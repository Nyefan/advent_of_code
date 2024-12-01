use std::error::Error;
use crate::day_3::{count};

pub fn main() -> Result<(), Box<dyn Error>> {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let encounters: Vec<_> = slopes
        .iter()
        .map(count)
        .collect();
    let product: usize = encounters.iter().product();
    println!(
        "day_03::part_2:\tTrees encountered: {} * {} * {} * {} * {} = {}",
        encounters[0], encounters[1], encounters[2], encounters[3], encounters[4], product
    );
    Ok(())
}
