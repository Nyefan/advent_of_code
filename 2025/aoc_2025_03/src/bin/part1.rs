use aoc_2025_03::part1::process;
use aoc_2025_03::Result;

#[tracing::instrument]
pub fn main() -> Result<'static, ()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../data/part1.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_2025_03::part1::process;

    #[test]
    fn test() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", process(input).unwrap());
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../data/part1.txt");
        assert_eq!("17158", process(input).unwrap());
    }
}
