use super::PasswordSpecification;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("day_02::part_1:\tValid password count: {}", super::count(validator)?);
    Ok(())
}

fn validator(ps: &PasswordSpecification) -> bool {
    let count = ps.password.chars().filter(|c| c.eq(&ps.letter)).count() as i32;
    return count >= ps.range.0 && count <= ps.range.1;
}
