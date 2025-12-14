use aoc_2025_02::*;
use std::time::Duration;

fn main() {
    divan::main();
}

#[divan::bench(min_time = Duration::from_millis(10), max_time = 1)]
fn _part2_naive() {
    // unwrap in tests
    part2::process_naive(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}

#[divan::bench(min_time = Duration::from_millis(10), max_time = 1)]
fn part2_better_algorithm() {
    // unwrap in tests
    part2::process(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}

#[divan::bench(min_time = Duration::from_millis(10), max_time = 1)]
fn part2_flatmap() {
    // unwrap in tests
    part2::process_flatmap(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}

#[divan::bench(min_time = Duration::from_millis(10), max_time = 1)]
fn part2_flatmap_numeric() {
    // unwrap in tests
    part2::process_flatmap_numeric(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}
