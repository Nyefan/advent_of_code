use aoc_2025_01::part2::process;
use aoc_2025_01::Result;

#[tracing::instrument]
pub fn main() -> Result<'static, ()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../data/part2.txt");
    let result = process(file)?;
    println!("{}", &result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_2025_01::part2::process;

    #[test]
    fn test_sample_input() {
        let input = "L68
L30
R48
R100
R500
L100
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("13", process(input).unwrap());
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../data/part2.txt");
        assert_eq!("6638", process(input).unwrap());
    }
}
