use num_traits::Num;
use vek::Vec2;

use core::cmp::Ordering;
use core::hint::unreachable_unchecked;
use core::ops::Index;

use crate::{Direction, Puzzle};

pub struct VecPuzzle<T: Num> {
    width: usize,
    height: usize,
    pieces: Vec<T>,
}

impl Default for VecPuzzle<u8> {
    fn default() -> Self {
        Self {
            width: 4,
            height: 4,
            pieces: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0],
        }
    }
}

impl<T: Num> Index<(usize, usize)> for VecPuzzle<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pieces[x * self.width + y]
    }
}

impl<T: Num + Copy> Puzzle<T> for VecPuzzle<T> {
    fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn index_of(&self, value: T) -> Option<(usize, usize)> {
        self.pieces
            .iter()
            .position(|&x| x == value)
            .map(|idx| (idx % self.width, idx / self.width))
    }

    fn slide_from(&mut self, from: (usize, usize)) -> Option<usize> {
        let from: Vec2<_> = from.into();
        let empty: Vec2<_> = self
            .index_of(T::zero())
            .expect("could not find an empty piece")
            .into();

        // e.g) ordering.0 == Less if from.x < empty.x
        #[rustfmt::skip]
        let ordering = (
            Ord::cmp(&from.x, &empty.x),
            Ord::cmp(&from.y, &empty.y)
        );

        #[rustfmt::skip]
        let ordering_equal = (
            ordering.0 == Ordering::Equal,
            ordering.1 == Ordering::Equal
        );

        let distance = match ordering_equal {
            // Should it just be 0 instead of None | Some(0)?
            (false, false) => return None,
            (true, true) => return Some(0),

            // y (outer index) is aligned; `copy_within`-optimized swapping
            (false, true) => {
                let row = &mut self.pieces[from.y * self.width..(from.y + 1) * self.width];

                use core::cmp::Ordering::*;
                match ordering.0 {
                    // |_|a|b|c|
                    //        ^
                    Greater => row.copy_within(empty.x + 1..=from.x, empty.x),

                    // |a|b|c|_|
                    //  ^
                    Less => row.copy_within(from.x..=empty.x - 1, from.x + 1),

                    // SAFETY: matched above
                    Equal => unsafe { unreachable_unchecked() },
                }

                from.x.abs_diff(empty.x)
            }

            // x (inner index) is not aligned; ordinary swapping using loop
            (true, false) => {
                use core::cmp::Ordering::*;
                let direction = match ordering.1 {
                    Greater => Direction::Up,
                    Less => Direction::Down,

                    // SAFETY: matched above
                    Equal => unsafe { unreachable_unchecked() },
                };

                let mut tmp = empty;
                while tmp != from {
                    self.pieces[from.y * self.width + from.x] = T::zero();
                    tmp += direction;
                }

                from.y.abs_diff(empty.y)
            }
        };

        self.pieces[from.y * self.width + from.x] = T::zero();

        Some(distance)
    }

    fn slide_towards(&mut self, direction: crate::Direction, distance: usize) -> Option<usize> {
        ignore::ignore! {
            |1|2|3|
            |4|5|6|
            |7|8|0|

            slide_towards(::Down, 2) {
                let from
                    = index_of(0) + (::Down) * -distance
                    = (2, 2) + (0, 1) * -2
                    = (2, 0);

                slide_from(from)
            }

            |1|2|0|
            |4|5|3|
            |7|8|6|
        }

        let zero = self
            .index_of(T::zero())
            .expect("could not find an empty piece");
        let direction: Vec2<isize> = direction.into();

        // At this point I don't care about the aesthetics anymore.
        self.slide_from((
            (zero.0 as isize).saturating_sub(direction.x.saturating_mul(distance as isize))
                as usize,
            (zero.1 as isize).saturating_sub(direction.y.saturating_mul(distance as isize))
                as usize,
        ))
    }
}
