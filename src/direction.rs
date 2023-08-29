use core::ops::AddAssign;

use vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Vec2<isize> {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
        .into()
    }
}

impl AddAssign<Direction> for Vec2<usize> {
    fn add_assign(&mut self, rhs: Direction) {
        let Vec2 { x, y } = rhs.into();

        // I absolutely love the fact that I eventually have to write these
        // dirty casting shenanigans somewhere in my code.
        (self.x, self.y) = (
            (self.x as isize).saturating_add(x) as usize,
            (self.y as isize).saturating_add(y) as usize,
        )
    }
}
