use {{crate_name}}::part2::process;
use {{crate_name}}::Result;

#[tracing::instrument]
pub fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../data/part2.txt");
    let result = process(file)?;
    println!("{:?}", result);
    Ok(())
}
