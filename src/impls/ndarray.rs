use ndarray::{array, s, Array2};
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
pub struct NdArrayPuzzle<T: Piece> {
    inner: Array2<T>,
}

impl Default for NdArrayPuzzle<u8> {
    fn default() -> Self {
        Self {
            #[rustfmt::skip]
            inner: array![
                [1, 2, 3, 4],
                [5, 6, 7, 8],
                [9, 10, 11, 0],
            ],
        }
    }
}

impl<T: Piece + Debug> Debug for NdArrayPuzzle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "StackPuzzle [")?;

        for row in self.inner.rows() {
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

            writeln!(f, "],")?;
        }

        write!(f, "]")
    }
}

impl<T: Piece> Index<(usize, usize)> for NdArrayPuzzle<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.inner.index((x, y))
    }
}

impl<T: Piece> Puzzle<T> for NdArrayPuzzle<T> {
    fn shape(&self) -> (usize, usize) {
        let shape = self.inner.shape();
        (shape[1], shape[0])
    }

    fn index_of(&self, value: T) -> Option<(usize, usize)> {
        self.inner
            .indexed_iter()
            .find(|(_, x)| **x == value)
            .map(|((y, x), _)| (x, y))
    }

    fn slide_from(&mut self, from: (usize, usize)) -> Option<usize> {
        let shape = self.shape();
        if !matches!((from, shape), ((x, y), (w, h)) if x < w && y < h) {
            return None;
        }

        let empty = self
            .index_of(num::zero())
            .expect("potential BUG: could not find an empty piece");

        // e.g) ordering.0 == Less if from.0 < empty.0
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
                use core::cmp::Ordering::*;

                match ordering.0 {
                    //        v from
                    // |_|a|b|c|
                    //  ^ empty
                    Greater => {
                        let mut slice = self.inner.slice_mut(s![empty.1, empty.0..=from.0]);
                        let slice = slice.as_slice_memory_order_mut();
                        unsafe { slice.unwrap_unchecked() }.rotate_left(1);
                    }

                    //  v from
                    // |a|b|c|_|
                    //        ^ empty
                    Less => {
                        let mut slice = self.inner.slice_mut(s![empty.1, from.0..=empty.0]);
                        let slice = slice.as_slice_memory_order_mut();
                        unsafe { slice.unwrap_unchecked() }.rotate_right(1);
                    }

                    // SAFETY: matched above in the definition of `ordering_equal`
                    Equal => unsafe { unreachable_unchecked() },
                }

                from.0.abs_diff(empty.0)
            }

            // x (inner index) is not aligned; ordinary swapping using loop
            (true, false) => {
                let distance = from.1.abs_diff(empty.1);

                use core::cmp::Ordering::*;
                let mut slice = match ordering.1 {
                    // a < empty
                    // b
                    // c
                    // _ < from
                    Greater => self.inner.slice_mut(s![empty.1..=from.1, empty.0]),

                    // a < from
                    // b
                    // c
                    // _ < empty
                    Less => self.inner.slice_mut(s![from.1..=empty.1; -1, empty.0]),

                    // SAFETY: matched above in the definition of `ordering_equal`
                    Equal => unsafe { unreachable_unchecked() },
                };
                let mut slice = slice.iter_mut();

                let mut cursor = unsafe { slice.next().unwrap_unchecked() };
                for next in slice {
                    mem::swap(cursor, next);
                    cursor = next;
                }

                distance
            }
        };

        Some(distance)
    }
}

impl<T: Piece + Display + Eq> Display for NdArrayPuzzle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (width, height) = self.shape();
        let digits = ((width * height - 1) as f32).log10() as usize + 1;
        for row in self.inner.rows() {
            for piece in row {
                if *piece == T::zero() {
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

impl<T: Piece> NdArrayPuzzle<T> {
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
            inner: Array2::from_shape_vec((height, width), pieces).unwrap(),
        }
    }

    pub fn random((width, height): (usize, usize)) -> Self {
        Self::random_with_rng(&mut rand::thread_rng(), (width, height))
    }
}
