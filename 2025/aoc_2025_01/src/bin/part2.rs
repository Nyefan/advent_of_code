use aoc_2025_01::part2::process;
use aoc_2025_01::Result;

#[tracing::instrument]
pub fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../data/part2.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_2025_01::part2::process;

    #[test]
    fn test() {
        let input = "";
        assert_eq!("", process(input).unwrap());
    }
}
