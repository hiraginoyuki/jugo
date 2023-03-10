use ndarray::prelude::*;
use num_traits::Zero;
use vek::Vec2;

use std::{fmt, ops::Index, hint::unreachable_unchecked, cmp::Ordering};

mod direction;
use crate::direction::Direction;

////////////////////

mod testoooooo {
    trait SlideUnchecked {
        unsafe fn slide_unchecked(&self);
    }
    trait Slide {
        fn slide_from(&self);
        fn slide_to(&self);
    }
    impl<P: SlideUnchecked> Slide for P {
        fn slide_from(&self) { unsafe { self.slide_unchecked() } }
        fn slide_to(&self) { unsafe { self.slide_unchecked() } }
    }

    // A
    struct PuzzleImplA;
    impl Slide for PuzzleImplA {
        fn slide_from(&self) { unimplemented!() }
        fn slide_to(&self) { unimplemented!() }
    }

    // B
    struct PuzzleImplB;
    impl SlideUnchecked for PuzzleImplB {
        unsafe fn slide_unchecked(&self) { unimplemented!() }
    }
}

pub trait Puzzle<T> {
    fn shape(&self) -> Vec2<usize>;

    fn slide_from(&mut self, idx: Vec2<usize>);
    fn slide_to(&mut self, direction: Direction, num_pieces: usize);
}

////////////////////

/// `puzzle[y][x]` :sob:
pub struct StackPuzzle<
    const WIDTH: usize,
    const HEIGHT: usize,
    T,
> {
    inner: [[T; WIDTH]; HEIGHT],
}

impl<
    const WIDTH: usize,
    const HEIGHT: usize,
    T,
    Idx: Into<Vec2<usize>>
> Index<Idx> for StackPuzzle<WIDTH, HEIGHT, T> {
    type Output = T;

    fn index(&self, idx: Idx) -> &Self::Output {
        let Vec2 { x, y } = idx.into();
        &self.inner[y][x]
    }
}

impl Default for StackPuzzle<4, 4, u8> {
    fn default() -> Self {
        Self {
            inner: [
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15,  0],
            ],
        }
    }
}

impl<
    const WIDTH: usize,
    const HEIGHT: usize,
    T: Zero,
> StackPuzzle<WIDTH, HEIGHT, T> {
    // unsafe fn slide_unchecked(&mut self, idx: Vec2<usize>, how_many_pieces_to_move: usize) {
    // 
    // }

    // TODO: shape? size? dimension(s)?
    #[inline]
    pub const fn shape(&self) -> Vec2<usize> {
        Vec2 { x: WIDTH, y: HEIGHT }
    }

    // TODO: idk it looks dirty; maybe separate into IntoIter<T>
    pub fn find_empty(&self) -> Vec2<usize> {
        for (y, row_pieces) in self.inner.iter().enumerate() {
            for (x, piece) in row_pieces.iter().enumerate() {
                if T::is_zero(piece) {
                    return Vec2 { x, y };
                }
            }
        }

        unreachable!("no empty slot found")
    }

    /// returns if the operation succeeded in `bool`.
    pub fn slide_from(&mut self, from: impl Into<Vec2<usize>>) -> bool
    where
        T: Copy + Ord + Zero,
    {
        let from = from.into();
        let empty = self.find_empty();

        // e.g) ordering.0 == Less if from.x < empty.x
        let ordering = (
            Ord::cmp(&from.x, &empty.x),
            Ord::cmp(&from.y, &empty.y),
        );
        let ordering_equal = (
            ordering.0 == Ordering::Equal,
            ordering.1 == Ordering::Equal,
        );

        match ordering_equal {
            (false, false) | (true, true) => return false,

            // y(outer index) is aligned; `copy_within`-optimized swapping
            (false, true) => {
                let row = &mut self.inner[from.y];

                use core::cmp::Ordering::*;
                match ordering.0 {
                    // |_|a|b|c|
                    //        ^
                    Greater => row.copy_within(empty.x + 1 ..= from.x, empty.x),

                    // |a|b|c|_|
                    //  ^
                    Less => row.copy_within(from.x ..= empty.x - 1, from.x + 1),

                    // SAFETY: matched above
                    Equal => unsafe { unreachable_unchecked() }
                }
            }

            // x(inner index) is aligned; ordinary swapping
            (true, false) => {
                use core::cmp::Ordering::*;
                let direction = match ordering.1 {
                    Greater => Direction::Up,
                    Less => Direction::Down,

                    // SAFETY: matched above
                    Equal => unsafe { unreachable_unchecked() }
                };

                let mut tmp = empty;
                while tmp != from {
                    self.inner[tmp.y][tmp.x] = self[tmp + direction];

                    tmp = tmp + direction;
                }
            }
        }

        self.inner[from.y][from.x] = T::zero();

        true
    }
}

impl<
    const WIDTH: usize,
    const HEIGHT: usize,
    T: Copy + fmt::Debug,
> fmt::Debug for StackPuzzle<WIDTH, HEIGHT, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !f.alternate() {
            return write!(f, "{:?}", self.inner);
        }

        write!(f, "[")?;

        let mut iter = self.inner.iter().peekable();
        while let Some(row) = iter.next() {
            write!(f, "{:?}", row)?;
            if iter.peek().is_some() {
                write!(f, ",\n ")?;
            }
        }

        write!(f, "]")
    }
}

////////////////////

pub struct NdArrayPuzzle<T> {
    inner: Array2<T>,
}

impl Default for NdArrayPuzzle<u8> {
    fn default() -> Self {
        Self {
            inner: array![
                [ 1,  2,  3,  0],
                [ 5,  6,  7,  4],
                [ 9, 10, 11,  8],
                [13, 14, 15, 12],
            ],
        }
    }
}

impl<T: Copy + PartialEq + Zero> NdArrayPuzzle<T> {
    pub fn find_zero(&self) -> Option<Vec2<usize>> {
        let z = T::zero();
        self.inner
            .indexed_iter()
            .find(|(_, &v)| v == z)
            .map(|((y, x), _)| Vec2 { x, y })
    }
}

impl<T: Copy + fmt::Display> fmt::Debug for NdArrayPuzzle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
