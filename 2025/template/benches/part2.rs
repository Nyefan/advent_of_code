use {{crate_name}}::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    // unwrap in tests
    part2::process(divan::black_box(include_str!("../data/part2.txt")))?
}
