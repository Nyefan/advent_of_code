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
