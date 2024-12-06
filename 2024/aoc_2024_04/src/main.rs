use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input: Vec<Vec<char>> = common::parse_file(&args[1])?;
    let part_1 = part_1(&input)?;
    let part_2 = part_2(&input)?;
    println!("part_1: {}", part_1);
    println!("part_2: {}", part_2);
    Ok(())
}

fn part_1(input: &Vec<Vec<char>>) -> Result<i32> {
  todo!()
}

fn part_2(input: &Vec<Vec<char>>) -> Result<i32> {
    todo!()
}

