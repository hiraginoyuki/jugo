use num_traits::Num;

use std::ops::Index;

mod direction;
pub use direction::Direction;

mod impls;
pub use impls::vec::VecPuzzle;

pub trait Puzzle<T: Num>: Index<(usize, usize)> {
    fn shape(&self) -> (usize, usize);
    fn index_of(&self, value: T) -> Option<(usize, usize)>;

    fn slide_from(&mut self, from: (usize, usize)) -> Option<usize>;
    fn slide_towards(&mut self, direction: Direction, distance: usize) -> Option<usize>;
}
