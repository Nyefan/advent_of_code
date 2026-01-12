use aoc_2025_04::Result;
use aoc_2025_04::part2::process;

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
    use aoc_2025_04::part2::process_fast_sliding_window;

    #[test]
    fn test() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(43, process_fast_sliding_window(input).unwrap());
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../data/part2.txt");
        assert_eq!(8890, process_fast_sliding_window(input).unwrap());
    }
}
