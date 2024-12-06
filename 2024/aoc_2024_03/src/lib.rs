use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &str) -> Result<i32> {
    let re =
        Regex::new(r"(?<operation>mul)\((?<first_operand>\d+),(?<second_operand>\d+)\)").unwrap();
    let result = re
        .captures_iter(input)
        .map(|c| c.extract().1)
        .map(|o: [&str; 3]| o[1].parse::<i32>().unwrap() * o[2].parse::<i32>().unwrap())
        .sum();
    Ok(result)
}

pub fn part_2(input: &str) -> Result<i32> {
    let valid_operations = ("do()".to_string() + input)
        .split("don't()")
        .map(|s| s.split_once("do()").unwrap_or(("", "")).1)
        .collect::<String>();
    part_1(&valid_operations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let lines_sample = std::fs::read_to_string("data/sample1")?;
        let result_sample = part_1(&lines_sample)?;
        assert_eq!(result_sample, 161);
        let lines_actual = std::fs::read_to_string("data/actual")?;
        let result_actual = part_1(&lines_actual)?;
        assert_eq!(result_actual, 178794710);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let lines_sample = std::fs::read_to_string("data/sample2")?;
        let result_sample = part_2(&lines_sample)?;
        assert_eq!(result_sample, 48);
        let lines_actual = std::fs::read_to_string("data/actual")?;
        let result_actual = part_2(&lines_actual)?;
        assert_eq!(result_actual, 76729637);
        Ok(())
    }
}
