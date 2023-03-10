use jugo::*;

fn main() {
    let mut p = StackPuzzle::default();

    println!("{:#?}", p);

    p.slide_from((1, 3));
    println!("{:#?}", p);

    p.slide_from((1, 1));
    println!("{:#?}", p);

    p.slide_from((3, 1));
    println!("{:#?}", p);

    p.slide_from((3, 3));
    println!("{:#?}", p);
}
