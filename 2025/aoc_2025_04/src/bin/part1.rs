use aoc_2025_04::Result;
use aoc_2025_04::part1::process;

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
    use aoc_2025_04::part1::process;

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
        assert_eq!(13, process(input).unwrap());
    }

    #[test]
    fn test_one_line() {
        let input = "..@@.@@@@.";
        assert_eq!(6, process(input).unwrap());
    }

    ///
    /// ..@@.@@@@.
    /// @@@.@.@.@@
    #[test]
    fn test_two_lines() {
        let input = "..@@.@@@@.\n@@@.@.@.@@";
        assert_eq!(12, process(input).unwrap());
    }

    ///
    /// @@
    /// @.
    #[test]
    fn test_small() {
        let input = "@@\n@.";
        assert_eq!(3, process(input).unwrap());
    }

    ///
    /// @@@
    /// @@.
    /// @..
    #[test]
    fn test_three() {
        let input = "@@@\n@@.\n@..";
        assert_eq!(3, process(input).unwrap());
    }

    ///
    /// @@@
    /// .@@
    /// ..@
    #[test]
    fn test_three_upper_right() {
        let input = "@@@\n.@@\n..@";
        assert_eq!(3, process(input).unwrap());
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../data/part1.txt");
        assert_eq!(1489, process(input).unwrap());
    }
}
