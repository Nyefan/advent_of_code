use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;
    let part_1 = part_1(&input)?;
    let part_2 = part_2(&input)?;
    println!("part_1: {}", part_1);
    println!("part_2: {}", part_2);
    Ok(())
}

fn part_1(input: &str) -> Result<i32> {
    let re =
        Regex::new(r"(?<operation>mul)\((?<first_operand>\d+),(?<second_operand>\d+)\)").unwrap();
    let result = re
        .captures_iter(input)
        .map(|c| c.extract().1)
        .map(|o: [&str; 3]| o[1].parse::<i32>().unwrap() * o[2].parse::<i32>().unwrap())
        .sum();
    Ok(result)
}

fn part_2(input: &str) -> Result<i32> {
    let valid_operations = ("do()".to_string() + input)
        .split("don't()")
        .map(|s| s.split_once("do()").unwrap_or(("", "")).1)
        .collect::<String>();
    part_1(&valid_operations)
}
