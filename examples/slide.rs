use jugo::{Puzzle, StackPuzzle, Direction::*};

fn main() {
    let mut p = StackPuzzle::default();
    println!("{:?}", p);

    p.slide_from((0, 3));
    println!("{:?}", p);

    p.slide_towards(Left, 2);
    println!("{:?}", p);

    p.slide_towards(Down, 2);
    println!("{:?}", p);

    p.slide_from((3, 1));
    println!("{:?}", p);

    p.slide_from((3, 3));
    println!("{:?}", p);
}
