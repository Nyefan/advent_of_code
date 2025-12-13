use {{crate_name}}::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    // unwrap in tests
    part1::process(divan::black_box(include_str!("../data/part1.txt"))).unwrap();
}
