use super::TreeRow;
use crate::utils::read_lines;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let count = read_lines("./inputs/3.input")?
        .flat_map(|line| line?.parse::<TreeRow>())
        .enumerate()
        .filter(|e| e.1[&(e.0 * 3)])
        .count();
    println!("Trees encountered: {}", count);
    Ok(())
}
