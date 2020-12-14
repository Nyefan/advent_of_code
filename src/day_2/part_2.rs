use super::PasswordSpecification;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Valid Password Count: {}", super::count(validator)?);
    Ok(())
}

fn validator(ps: &PasswordSpecification) -> bool {
    fn get_char(ps: &PasswordSpecification, index: i32) -> Option<char> {
        ps.password.chars().nth(index as usize)
    }
    let first = get_char(ps, ps.range.0 - 1) == Some(ps.letter);
    let second = get_char(ps, ps.range.1 - 1) == Some(ps.letter);
    first ^ second
}
