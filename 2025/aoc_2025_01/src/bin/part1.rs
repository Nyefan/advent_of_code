#[tracing::instrument]
pub fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../data/input1.txt");
    let result = process(file)?;
    println!("{}", result);

}

pub(crate) fn process(input: impl AsRef<str>) -> Result<()> {
    todo!("aoc_2025_01 part 1 is not implemented yet")
}