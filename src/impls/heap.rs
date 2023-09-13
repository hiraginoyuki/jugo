use num::Integer;
use omniswap::{rotate, swap};
use rand::{seq::SliceRandom, Rng};

use core::fmt::{self, Debug, Display};
use core::hint::unreachable_unchecked;
use core::mem;
use core::ops::Index;
use core::{cmp::Ordering, iter::once};

use crate::{is_solvable, Piece, Puzzle};

#[derive(Clone)]
pub struct BoxPuzzle<T: Piece> {
    inner: Box<[T]>,
    width: usize,
}

impl Default for BoxPuzzle<u8> {
    fn default() -> Self {
        Self {
            width: 4,
            #[rustfmt::skip]
            inner: [
                1, 2, 3, 4,
                5, 6, 7, 8,
                9, 10, 11, 12,
                13, 14, 15, 0,
            ].into(),
        }
    }
}

impl<T: Piece + Debug> Debug for BoxPuzzle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BoxPuzzle [")?;

        for row in self.inner.chunks(self.width) {
            write!(f, "  [")?;

            let mut first = true;
            for piece in row {
                if first {
                    first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", piece)?;
            }

            ignore::ignore!(The quick brown fox jumps);

            writeln!(f, "],")?;
        }
        write!(f, "]")
    }
}

impl<T: Piece> Index<(usize, usize)> for BoxPuzzle<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.inner[x * self.width + y]
    }
}

// #[derive(Debug)]
// struct Iter<'a, T: 'a> {
//     inner: core::slice::Iter<'a, T>,
// }
// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.inner.next()
//     }
// }

impl<T: Piece> Puzzle<T> for BoxPuzzle<T> {
    // type Iter<'a> = Iter<'a, T> where T: 'a;
    // fn iter(&self) -> Self::Iter<'_> {
    //     Iter {
    //         inner:self.inner.iter()
    //     }
    // }

    #[inline]
    fn shape(&self) -> (usize, usize) {
        (self.width, self.inner.len() / self.width)
    }

    fn index_of(&self, value: T) -> Option<(usize, usize)> {
        self.inner
            .iter()
            .position(|x| *x == value)
            .map(|idx| (idx % self.width, idx / self.width))
    }

    fn slide_from(&mut self, from: (usize, usize)) -> Option<usize> {
        let (width, height) = self.shape();
        if !matches!(from, (x, y) if x < width && y < height) {
            return None;
        }

        let empty = self
            .index_of(num::zero())
            .expect("potential BUG: could not find an empty piece");

        // Ord::cmp(&1, &0) == Ordering::Greater
        // Ord::cmp(&1, &1) == Ordering::Equal
        // Ord::cmp(&1, &2) == Ordering::Less
        #[rustfmt::skip]
        let ordering = (
            Ord::cmp(&from.0, &empty.0),
            Ord::cmp(&from.1, &empty.1)
        );

        #[rustfmt::skip]
        let ordering_equality = (
            ordering.0 == Ordering::Equal,
            ordering.1 == Ordering::Equal
        );

        let distance = match ordering_equality {
            // Should it just be 0 instead of None | Some(0)?
            (false, false) => return None,
            (true, true) => return Some(0),

            // y (outer index) is aligned; `copy_within`-optimized swapping
            (false, true) => {
                let row = &mut self.inner[from.1 * self.width..(from.1 + 1) * self.width];

                use core::cmp::Ordering::*;
                match ordering.0 {
                    // |_|a|b|c|
                    //        ^
                    Greater => row[empty.0..=from.0].rotate_left(1),

                    // |a|b|c|_|
                    //  ^
                    Less => row[from.0..=empty.0].rotate_right(1),

                    // SAFETY: matched above in the definition of `ordering_equal`
                    Equal => unsafe { unreachable_unchecked() },
                }

                from.0.abs_diff(empty.0)
            }

            // x (inner index) is not aligned; ordinary swapping using loop
            (true, false) => {
                let distance = from.1.abs_diff(empty.1);

                let mut iterators = (None, None);

                let column = self
                    .inner
                    .chunks_mut(self.width)
                    .map(|row| &mut row[from.0]);
                use core::cmp::Ordering::*;
                let column: &mut dyn Iterator<Item = _> = match ordering.1 {
                    Less => iterators
                        .0
                        .insert(column.skip(from.1).take(distance + 1).rev()),
                    Greater => iterators.1.insert(column.skip(empty.1).take(distance + 1)),

                    // SAFETY: matched above in the definition of `ordering_equal`
                    Equal => unsafe { unreachable_unchecked() },
                };

                let mut cursor = unsafe { column.next().unwrap_unchecked() };
                for next in column {
                    mem::swap(cursor, next);
                    cursor = next;
                }

                distance
            }
        };

        Some(distance)
    }
}

impl<T: Piece + Display> Display for BoxPuzzle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digits = ((self.inner.len() - 1) as f32).log10() as usize + 1;
        for row in self.inner.chunks(self.width) {
            for piece in row {
                if piece.is_zero() {
                    write!(f, "{: >digits$} ", "")?;
                } else {
                    write!(f, "{: >digits$} ", piece)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Piece> BoxPuzzle<T> {
    pub fn random_with_rng(rng: &mut (impl Rng + ?Sized), (width, height): (usize, usize)) -> Self {
        let len = width * height;
        let mut pieces: Vec<T> = (1_usize..len)
            .chain(once(0))
            .map(num::cast)
            .collect::<Option<_>>()
            .expect("could not cast pieces to usize");

        pieces[..len - 1].shuffle(rng);

        if !is_solvable(&pieces, width) {
            pieces.swap(0, 1);
        }

        let empty_idx = rng.gen_range(0..len);
        let empty_pos = (empty_idx % width, empty_idx / width);

        match (width - 1 - empty_pos.0) + (height - 1 - empty_pos.1) {
            0 => {}
            d if d.is_odd() => {
                swap!(&mut pieces[empty_idx], &mut pieces[len - 1]);
            }
            _ => {
                rotate!(
                    &mut pieces[empty_idx],
                    &mut pieces[len - 2],
                    &mut pieces[len - 1]
                );
            }
        }

        Self {
            inner: pieces.into_boxed_slice(),
            width,
        }
    }

    pub fn random((width, height): (usize, usize)) -> Self {
        Self::random_with_rng(&mut rand::thread_rng(), (width, height))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    pub fn iter_indexed(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.inner
            .iter()
            .enumerate()
            .map(move |(idx, piece)| ((idx % self.width, idx / self.width), piece))
    }

    pub fn is_solved(&self) -> bool {
        let iter_current = self.inner.iter();
        let iter_solved = (1..self.inner.len()).chain(once(0));

        for (current, solved) in iter_current.zip(iter_solved) {
            if num::cast(current.clone()) != Some(solved) {
                return false;
            }
        }

        true
    }
}
