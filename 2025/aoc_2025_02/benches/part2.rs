use aoc_2025_02::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part2() {
    // unwrap in tests
    part2::process(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}

#[divan::bench]
fn part2_naive() {
    // unwrap in tests
    part2::process_naive(divan::black_box(include_str!("../data/part2.txt"))).unwrap();
}
