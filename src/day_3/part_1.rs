use std::error::Error;
use crate::day_3::count;

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Trees encountered: {}", count(&(3, 1)));
    Ok(())
}
