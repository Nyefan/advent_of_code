use aoc_2025_02::part2::process_flatmap;
use aoc_2025_02::Result;

#[tracing::instrument]
pub fn main() -> Result<'static, ()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../data/part2.txt");
    let result = process_flatmap(file)?;
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_2025_02::part2::process_flatmap;

    #[test]
    fn test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(4174379265, process_flatmap(input).unwrap());
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../data/part2.txt");
        assert_eq!(54446379122, process_flatmap(input).unwrap());
    }
}
