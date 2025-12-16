use aoc_2025_03::part2::process;
use aoc_2025_03::Result;

#[tracing::instrument]
pub fn main() -> Result<'static, ()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../data/part2.txt");
    let result = process(file)?;
    println!("{:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_2025_03::part2::{find_max_joltage_for_line, process};
    use rstest::rstest;

    #[test]
    fn test() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("3121910778619", process(input).unwrap());
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_line_1(#[case] input: &str, #[case] result: u64) {
        assert_eq!(result, find_max_joltage_for_line(input));
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../data/part2.txt");
        assert_eq!("170449335646486", process(input).unwrap());
    }
}
