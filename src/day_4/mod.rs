use crate::utils::read_lines;
use passport::Passport;

pub mod part_1;
pub mod part_2;
mod passport;

lazy_static! {
    static ref PASSPORTS: Vec<Passport> = read_lines("./inputs/4.input")
        .unwrap()
        .flatten()
        .collect::<Vec<String>>()
        .join(" ")
        .split("  ")
        .flat_map(|line| line.parse::<Passport>())
        .collect();
}

// fn count(slope: &(usize, usize)) -> usize {
//     TREE_MAP
//         .iter()
//         .step_by(slope.1)
//         .enumerate()
//         .filter(|&e| e.1[&(e.0 * slope.0)])
//         .count()
// }
