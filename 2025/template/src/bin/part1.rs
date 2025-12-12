use {{crate_name}}::part1::process;
use {{crate_name}}::Result;

#[tracing::instrument]
pub fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../data/part1.txt");
    let result = process(file)?;
    println!("{:?}", result);
    Ok(())
}
