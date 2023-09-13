use num::{Integer, NumCast};
use omniswap::rotate;

mod direction;
pub use direction::Direction;

mod impls {
    pub mod heap;
    #[cfg(feature = "ndarray")]
    pub mod ndarray;
    pub mod stack;
}
pub use impls::heap::BoxPuzzle;
#[cfg(feature = "ndarray")]
pub use impls::ndarray::NdArrayPuzzle;
pub use impls::stack::StackPuzzle;

pub trait Piece: Clone + Integer + NumCast {}
impl<T: Clone + Integer + NumCast> Piece for T {}

pub trait Puzzle<T: Piece>: core::ops::Index<(usize, usize)> {
    fn shape(&self) -> (usize, usize);
    fn index_of(&self, value: T) -> Option<(usize, usize)>;

    // 0 1 2
    // 3 4 5
    // 6 7 8
    // type Iter<'a>: Iterator<Item = &'a T>
    // where Self: 'a, T: 'a;
    // fn iter(&self) -> Self::Iter<'_>;

    // (0,0) (1,0) (2,0)
    // (0,1) (1,1) (2,1)
    // (0,2) (1,2) (2,2)
    // type Iter2d: Iterator<Item = ((usize, usize), T)>;
    // fn iter_indexed_2d(&self) -> eru -Self::Iter2d;

    fn slide_from(&mut self, from: (usize, usize)) -> Option<usize>;
    fn slide_towards(&mut self, direction: Direction, distance: usize) -> Option<usize> {
        let (width, height) = self.shape();
        let zero = self
            .index_of(num::zero())
            .expect("potential BUG: could not find an empty piece");

        let direction: (isize, isize) = direction.into();

        self.slide_from((
            zero.0
                .saturating_add_signed(-direction.0.saturating_mul(distance as isize))
                .clamp(0, width),
            zero.1
                .saturating_add_signed(-direction.1.saturating_mul(distance as isize))
                .clamp(0, height),
        ))
    }
}

#[test]
fn is_solvable_works() {
    assert_eq!(is_solvable(&[1, 2, 3, 4, 5, 6, 7, 8, 0], 3), true);
    assert_eq!(is_solvable(&[2, 4, 8, 7, 6, 5, 3, 0, 1], 3), true);
    assert_eq!(is_solvable(&[2, 1, 3, 4, 8, 5, 0, 6, 7], 3), true);
    assert_eq!(is_solvable(&[1, 2, 3, 4, 5, 0, 7, 6, 8], 3), false);
    assert_eq!(is_solvable(&[2, 4, 8, 7, 0, 5, 3, 1, 6], 3), false);
    assert_eq!(is_solvable(&[2, 1, 3, 0, 8, 5, 4, 7, 6], 3), false);
}

#[allow(dead_code)]
pub(crate) fn is_solvable<T: Piece>(pieces: &[T], width: usize) -> bool {
    debug_assert!(width >= 2);
    debug_assert!(pieces.len() >= 4);
    debug_assert!(pieces.len() % width == 0);

    let mut pieces = pieces
        .iter()
        .cloned()
        .map(num::cast)
        .collect::<Option<Vec<usize>>>()
        .expect("could not cast pieces to usize");

    let height = pieces.len() / width;
    let last_idx = pieces.len() - 1;
    let Some(empty_idx) = pieces.iter().position(|&p| p == 0) else {
        return false;
    };
    let empty_pos = (empty_idx % width, empty_idx / width);

    match (width - 1 - empty_pos.0) + (height - 1 - empty_pos.1) {
        0 => {}
        d if d.is_odd() => {
            // odd, just swap with the last piece
            pieces.swap(empty_idx, last_idx);
        }
        _ => {
            // even, also swap with the second last piece
            rotate!(
                &mut pieces[empty_idx],
                &mut pieces[last_idx - 1],
                &mut pieces[last_idx]
            );
        }
    }

    let mut swaps: usize = 0;
    for i in 0..=pieces.len() - 3 {
        loop {
            let j = pieces[i] - 1;
            if i == j {
                break;
            }
            pieces.swap(i, j);
            swaps += 1;
        }
    }

    swaps.is_even()
}
