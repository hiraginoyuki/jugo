use num::Integer;
use omniswap::{rotate, swap};
use rand::{seq::SliceRandom, Rng};

use core::fmt::Debug;
use core::hint::unreachable_unchecked;
use core::ops::Index;
use core::{cmp::Ordering, iter::once};
use core::mem;
use std::fmt::Display;

use crate::{is_solvable, Puzzle, Piece};

#[derive(Clone)]
pub struct BoxPuzzle<T: Piece> {
    pieces: Box<[T]>,
    width: usize,
    height: usize,
}

impl Default for BoxPuzzle<u8> {
    fn default() -> Self {
        Self {
            width: 4,
            height: 4,
            #[rustfmt::skip]
            pieces: [
                1, 2, 3, 4,
                5, 6, 7, 8,
                9, 10, 11, 12,
                13, 14, 15, 0,
            ].into(),
        }
    }
}

impl<T: Piece + Debug> Debug for BoxPuzzle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "StackPuzzle [")?;

        for row in self.pieces.chunks(self.width) {
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

impl<T: Piece> Index<(usize, usize)> for BoxPuzzle<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pieces[x * self.width + y]
    }
}

impl<T: Piece> Puzzle<T> for BoxPuzzle<T> {
    fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn index_of(&self, value: T) -> Option<(usize, usize)> {
        self.pieces
            .iter()
            .position(|x| *x == value)
            .map(|idx| (idx % self.width, idx / self.width))
    }

    fn slide_from(&mut self, from: (usize, usize)) -> Option<usize> {
        if !matches!(from, (x, y) if x < self.width && y < self.height) {
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
                let row = &mut self.pieces[from.1 * self.width..(from.1 + 1) * self.width];

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

                let column = self.pieces.chunks_mut(self.width).map(|row| &mut row[from.0]);
                use core::cmp::Ordering::*;
                let column: &mut dyn Iterator<Item = _> = match ordering.1 {
                    Less => iterators.0.insert(column.skip(from.1).take(distance + 1).rev()),
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

impl<T: Piece + Display + Eq> Display for BoxPuzzle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digits = ((self.width * self.height - 1) as f32).log10() as usize + 1;
        for row in self.pieces.chunks(self.width) {
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

impl<T: Piece> BoxPuzzle<T> {
    ignore::ignore! {
      source: "https://github.com/hiraginoyuki/15-puzzle/blob/d2ec06a0809dce4f3c08ead4af36981ea3d8f902/src/puzzle.ts#L84-L106"

      public checkSolvable (): boolean {
        const cloned = new Puzzle(this, this.width)
        if (cloned.at(-1) !== 0) {
          cloned.tap(cloned.width - 1, floor(cloned.indexOf(0) / cloned.width))
          cloned.tap(cloned.width - 1, cloned.height - 1)
        }
        let isEven = true
        for (let currentIndex = 0, targetIndex, targetValue, assigneeValue; currentIndex < cloned.length - 1; currentIndex++) {
          targetValue = cloned[currentIndex]
          targetIndex = targetValue - 1
          if (currentIndex === targetIndex) continue
          while (true) {
            assigneeValue = cloned[targetIndex]
            cloned[targetIndex] = targetValue
            targetValue = assigneeValue
            targetIndex = targetValue - 1
            isEven = !isEven
            if (currentIndex === targetIndex) break
          }
          cloned[targetIndex] = targetValue
        }
        return isEven
      }
    }

    pub fn random_with_rng(
        rng: &mut (impl Rng + ?Sized),
        (width, height): (usize, usize),
    ) -> Option<Self> {
        let len = width * height;
        let mut pieces: Vec<T> = (1_usize..len)
            .chain(once(0))
            .map(num::cast)
            .collect::<Option<_>>()?;

        pieces[..len - 1].shuffle(rng);

        if !is_solvable(&pieces, width)? {
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

        Some(Self {
            width,
            height,
            pieces: pieces.into_boxed_slice(),
        })
    }

    pub fn random((width, height): (usize, usize)) -> Option<Self> {
        Self::random_with_rng(&mut rand::thread_rng(), (width, height))
    }
}
