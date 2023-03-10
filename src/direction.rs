use vek::Vec2;

use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Mul<isize> for Direction {
    type Output = Vec2::<isize>;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::Output::from(self) * rhs
    }
}

impl Add<Direction> for Vec2<usize> {
    type Output = Self;

    fn add(mut self, rhs: Direction) -> Self::Output {
        let hello = Vec2::<isize>::from(rhs);

        self.x = self.x.wrapping_add_signed(hello.x);
        self.y = self.y.wrapping_add_signed(hello.y);

        self
    }
}

impl From<Direction> for Vec2<isize> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up    => Vec2 { x:  0, y:  1 },
            Direction::Down  => Vec2 { x:  0, y: -1 },
            Direction::Left  => Vec2 { x: -1, y:  0 },
            Direction::Right => Vec2 { x:  1, y:  0 },
        }
    }
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
