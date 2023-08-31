use core::cmp::Ordering;
use core::fmt::Debug;
use core::hint::unreachable_unchecked;
use core::mem;
use core::ops::Index;

use crate::{Puzzle, Piece};

#[derive(Clone)]
pub struct StackPuzzle<const W: usize, const H: usize, T: Piece> {
    pieces: [[T; W]; H],
}

ignore::ignore! {
    impl<const W: usize, const H: usize, T> Debug for StackPuzzle<W, H, T>
    where
        T: num::Integer + Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if f.alternate() {
                write!(f, "{:#?}", self.pieces)
            } else {
                write!(f, "{:?}", self.pieces)
            }
        }
    }
}

impl<const W: usize, const H: usize, T: Piece + Debug> Debug for StackPuzzle<W, H, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "StackPuzzle [")?;

        for row in self.pieces.iter() {
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

impl Default for StackPuzzle<4, 4, u8> {
    fn default() -> Self {
        Self {
            #[rustfmt::skip]
            pieces: [
                [1, 2, 3, 4],
                [5, 6, 7, 8],
                [9, 10, 11, 12],
                [13, 14, 15, 0],
            ],
        }
    }
}

impl<const W: usize, const H: usize, T: Piece> Index<(usize, usize)> for StackPuzzle<W, H, T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pieces[y][x]
    }
}

impl<const W: usize, const H: usize, T: Piece> Puzzle<T> for StackPuzzle<W, H, T> {
    fn shape(&self) -> (usize, usize) {
        (W, H)
    }

    fn index_of(&self, value: T) -> Option<(usize, usize)> {
        self.pieces.iter().enumerate().find_map(|(row, row_slice)| {
            row_slice
                .iter()
                .position(|x| *x == value)
                .map(|col| (col, row))
        })
    }

    fn slide_from(&mut self, from: (usize, usize)) -> Option<usize> {
        use core::cmp::Ordering::*;

        if !matches!(from, (x, y) if x < W && y < H) {
            return None;
        }

        let empty = self
            .index_of(T::zero())
            .expect("potential BUG: could not find an empty piece");

        // e.g) ordering.0 == Less if from.x < empty.x
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
                let row = &mut self.pieces[from.1];

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

                let column = self.pieces.iter_mut().map(|row| &mut row[from.0]);
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
