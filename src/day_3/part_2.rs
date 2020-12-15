use super::TreeRow;
use crate::utils::read_lines;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let tree_map: Vec<_> = read_lines("./inputs/3.input")?
        .flat_map(|line| line?.parse::<TreeRow>())
        .collect();
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let encounters: Vec<_> = slopes
        .iter()
        .map(|slope| {
            tree_map
                .iter()
                .step_by(slope.1)
                .enumerate()
                // .inspect(|&e| println!("({}, \t{})", slope.1 * e.0, e.1))
                .filter(|&e| e.1[&(e.0 * slope.0)])
                .count()
        })
        .collect();
    let product: usize = encounters.iter().product();
    println!(
        "Trees encountered: {} * {} * {} * {} * {} = {}",
        encounters[0], encounters[1], encounters[2], encounters[3], encounters[4], product
    );
    Ok(())
}
