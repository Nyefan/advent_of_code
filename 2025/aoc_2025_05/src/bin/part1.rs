use aoc_2025_05::part1::process;
use aoc_2025_05::Result;

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
    use aoc_2025_05::part1::process;

    #[test]
    fn test() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!(3, process(input).unwrap());
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../data/part1.txt");
        assert_eq!(874, process(input).unwrap());
    }
}
