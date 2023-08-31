use criterion::*;
use jugo::{NdArrayPuzzle, Puzzle, Piece, BoxPuzzle};
use rand::{SeedableRng, thread_rng, Rng};
use rand_xoshiro::Xoshiro256StarStar;

fn bench_slide_from(c: &mut Criterion) {
    let mut rng = thread_rng();

    for shape in [(4, 4), (5, 7), (16, 16)] {
        c.bench_function(&format!("BoxPuzzle, {shape:?}"), raxrdxal(
            Xoshiro256StarStar::from_rng(&mut rng).unwrap(),
            shape,
            |rng, shape| BoxPuzzle::<u8>::random_with_rng(rng, shape).unwrap(),
        ));

        c.bench_function(&format!("NdArrayPuzzle, {shape:?}"), raxrdxal(
            Xoshiro256StarStar::from_rng(&mut rng).unwrap(),
            shape,
            |rng, shape| NdArrayPuzzle::<u8>::random_with_rng(rng, shape).unwrap(),
        ));
    }
}

fn raxrdxal<P, T, R>(
    mut rng: R,
    shape: (usize, usize),
    mut gen: impl FnMut(&mut R, (usize, usize)) -> P,
) -> impl FnMut(&mut Bencher)
where
    P: Puzzle<T>,
    T: Piece,
    R: Rng,
{
    move |bencher| {
        let setup = || {
            let puzzle = gen(&mut rng, shape);
            let empty = puzzle.index_of(T::zero()).unwrap();
            let idx = match rng.gen() {
                true => (rng.gen_range(0..shape.0), empty.1),
                false => (empty.0, rng.gen_range(0..shape.1)),
            };
            
            (puzzle, idx)
        };
        let routine = |(mut puzzle, idx)| P::slide_from(&mut puzzle, idx);

        bencher.iter_batched(setup, routine, BatchSize::SmallInput);
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(32768);
    targets = bench_slide_from
}
criterion_main!(benches);
