use std::time::Duration;
use aoc_2025_02::*;

fn main() {
    divan::main();
}

#[divan::bench(min_time = Duration::from_millis(10), max_time = 1)]
fn part2_naive() {
    // unwrap in tests
    part2::process_naive(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}

#[divan::bench(min_time = Duration::from_millis(10), max_time = 1)]
fn part2() {
    // unwrap in tests
    part2::process(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}

#[divan::bench(min_time = Duration::from_millis(10), max_time = 1)]
fn part2_flatmap() {
    // unwrap in tests
    part2::process_flatmap(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}
