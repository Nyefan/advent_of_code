use super::PasswordSpecification;
use crate::utils::read_lines;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let count = read_lines("./inputs/2.input")?
        .flat_map(|line| line?.parse::<PasswordSpecification>())
        .filter(validate)
        .count();
    println!("Valid Password Count: {}", count);
    Ok(())
}

fn validate(ps: &PasswordSpecification) -> bool {
    fn get_char(ps: &PasswordSpecification, index: i32) -> Option<char> {
        ps.password.chars().nth(index as usize)
    }
    let first = get_char(ps, ps.range.0 - 1) == Some(ps.letter);
    let second = get_char(ps, ps.range.1 - 1) == Some(ps.letter);
    first ^ second
}
