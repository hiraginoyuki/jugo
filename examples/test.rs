use std::{fmt, ops::RangeInclusive, hint::unreachable_unchecked};

use ndarray::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Index2D {
    x: usize,
    y: usize,
}
impl Index2D {
    fn to_tuple(self) -> (usize, usize) {
        (self.x, self.y)
    }
}
impl From<(usize, usize)> for Index2D {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}
impl From<Index2D> for (usize, usize) {
    fn from(index: Index2D) -> (usize, usize) {
        index.to_tuple()
    }
}

trait DebugExt: Sized + fmt::Debug {
    fn dbg(self) -> Self {
        dbg!(self)
    }
}
impl<T: fmt::Debug> DebugExt for T {}

pub(crate) type P = Array2<u8>;

trait Puzzle {
    type Piece;

    unsafe fn set(index: (usize, usize), value: Self::Piece);
    /// what args
    unsafe fn bulk_move();
}

fn _range_between_inclusive<N: num_traits::PrimInt>(a: N, b: N) -> RangeInclusive<N> {
    if a < b { a..=b } else { b..=a }
}

mod j {
    use std::fmt;
    use ndarray::prelude::*;

    use super::P;

    trait PExt {
        type D<'a>: fmt::Display + 'a where Self: 'a;

        fn display_in_correct_order(&self) -> Self::D<'_>;
        fn correct_order(&self) -> Self;
    }

    struct PDisplay<'a>(&'a P);
    impl<'a> fmt::Display for PDisplay<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for _lane in self.0.lanes(Axis(0)) {
                write!(f, "123123")?;
            };

            Ok(())
        }
    }
    impl PExt for P {
        type D<'a> = PDisplay<'a>;

        fn display_in_correct_order(&self) -> Self::D<'_> {
            PDisplay(self)
        }

        fn correct_order(&self) -> Self {
            let &[height, width] = self.shape() else { unreachable!() };
            Self::from_shape_fn((width, height), |(y, x)| self[(x, y)])
        }
    }
}

fn main() {
    let mut puzzle = dbg!(array![
        [ 1,  2,  3,  4],
        [ 5,  6,  7,  8],
        [ 9, 10, 11, 12],
        [13, 14, 15,  0],
     // [ 1,  5,  9, 13],
     // [ 2,  6, 10, 14],
     // [ 3,  7, 11, 15],
     // [ 4,  8, 12,  0],
    ].reversed_axes());

    //     0      1      2      3
    //   +------+------+------+------+
    // 0 |(0, 0)|(1, 0)|(2, 0)|(3, 0)|
    //   |   1  |   2  |   3  |   4  |
    //   +------+------+------+------+
    // 1 |(0, 1)|(1, 1)|(2, 1)|(3, 1)|
    //   |   5  |   6  |   7  |   8  |
    //   +------+------+------+------+
    // 2 |(0, 2)|(1, 2)|(2, 2)|(3, 2)|
    //   |   9  |  10  |  11  |  12  |
    //   +------+------+------+------+
    // 3 |(0, 3)|(1, 3)|(2, 3)|(3, 3)|
    //   |  13  |  14  |  15  |   0  |
    //   +------+------+------+------+

    // slide(&mut puzzle, (3, 0));
    dbg!(find_zero(&puzzle));

 // fn slido(target: (usize, usize)) -> bool
    {
        // (y, x) because ndarray
        let zero = (3, 3);
        let target = (1, 3);

        use std::cmp::Ordering::*;

        // lhs(target) is <[`Ordering`]> than/to rhs(zero)
        match (target.0.cmp(&zero.0), target.1.cmp(&zero.1)) {
            (x @ (Less | Greater), Equal) => {
                let range = match x {
                    Less => target.0..=zero.0,
                    Greater => zero.0..=target.0,
                    Equal => unsafe { unreachable_unchecked() },
                };

                let mut slice = puzzle.slice_mut(s![3, range]);
                slice.as_slice_mut().unwrap().copy_within(0..=1, 1);
                *puzzle.get_mut((3, 1)).unwrap() = 0;
            }
            _ => unimplemented!()
        }
    };

    dbg!(&puzzle);
}

fn find_zero(p: &P) -> Option<(usize, usize)> {
    p
        .indexed_iter()
        .find(|(_, &cell)| cell == 0)
        .map(|(idx, _)| idx)
}

/// Maybe it can be slided, maybe not.
fn _slide(p: &mut P, index: impl Into<Index2D>) -> bool {
    let index = index.into();

    let Some(_zero) = find_zero(p) else { return false };
    let Some(_target_value) = p.get(index.to_tuple()).dbg() else { return false };

    // p.shared_axes(coord, zero_idx);

    true
}
