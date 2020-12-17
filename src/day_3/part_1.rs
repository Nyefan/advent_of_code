use std::error::Error;
use crate::day_3::count;

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("day_03::part_1:\tTrees encountered: {}", count(&(3, 1)));
    Ok(())
}
