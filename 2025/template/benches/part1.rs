use {{crate_name}}::*;

fn main() -> Result<()> {
    divan::main();
}

#[divan::bench]
fn part1() -> Result<()> {
    part1::process(divan::black_box(include_str!("../data/part1.txt")))?
}
