use aoc_2025_02::*;

fn main() {
    divan::main();
}

// #[divan::bench]
// fn part1_naive() {
//     part1::process_naive(divan::black_box(include_str!("../data/part1.txt"))).unwrap();
// }

#[divan::bench]
fn part1() {
    // unwrap in tests
    part1::process(divan::black_box(include_str!("../data/part1.txt"))).unwrap();
}

#[divan::bench]
fn part1_flatmap() {
    part1::process_flatmap(divan::black_box(include_str!("../data/part1.txt"))).unwrap();
}
