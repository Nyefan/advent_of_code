use super::{PasswordSpecification, Validator};
use crate::utils::read_lines;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let count = read_lines("./inputs/2.input")?
        .flat_map(|line| line?.parse::<PasswordSpecification>())
        .filter(|pv| pv.validate())
        .count();
    println!("Valid Password Count: {}", count);
    Ok(())
}

impl Validator for PasswordSpecification {
    fn validate(&self) -> bool {
        let count = self.password.chars().filter(|c| c.eq(&self.letter)).count() as i32;
        return count >= self.range.0 && count <= self.range.1;
    }
}
