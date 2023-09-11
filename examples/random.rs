use std::thread;

use jugo::BoxPuzzle;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro128StarStar;

fn main() {
    let puzzle = BoxPuzzle::<u8>::random((4, 4));

    println!("{:?}", puzzle);

    let mut rng = Xoshiro128StarStar::seed_from_u64(123123);
    let puzzle_1 = BoxPuzzle::<u8>::random_with_rng(&mut rng, (4, 4));
    let mut rng = Xoshiro128StarStar::seed_from_u64(123123);
    let puzzle_2 = BoxPuzzle::<u8>::random_with_rng(&mut rng, (4, 4));

    println!("{puzzle_1:?},\n{puzzle_2:?}");

    let start = std::time::Instant::now();
    println!("begin");

    (0..dbg!(num_cpus::get())).map(|i| {
        let t = thread::spawn(|| {
            let mut rng = Xoshiro128StarStar::seed_from_u64(12312364);
            for _ in 0..=2u32.pow(22) {
                let _p = BoxPuzzle::<u8>::random_with_rng(&mut rng, (4, 4));
            }
        });

        println!("spawned {i}");

        (i, t)
    }).collect::<Vec<_>>().into_iter().for_each(|(i, t)| { 
        t.join().unwrap();
        println!("joined {i}");
    });

    println!("end: {:?}", start.elapsed());
}
