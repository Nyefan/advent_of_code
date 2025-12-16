use aoc_2025_03::*;
use std::time::Duration;

fn main() {
    divan::main();
}

#[divan::bench(min_time = Duration::from_millis(10), max_time = 1)]
fn _part2() {
    // unwrap in tests
    part2::process(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}
