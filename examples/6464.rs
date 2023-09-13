fn main() {
    let start = std::time::Instant::now();
    let p = jugo::BoxPuzzle::<u32>::random((64, 64));
    let elapsed = start.elapsed();
    println!("{p}\n{elapsed:?}");
}
