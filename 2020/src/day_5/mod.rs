use crate::utils::read_lines;
use boarding_pass::BoardingPass;

mod boarding_pass;
pub mod part_1;
pub mod part_2;

lazy_static! {
    static ref BOARDINGPASSES: Vec<BoardingPass> = read_lines("./inputs/5.input")
        .unwrap()
        .flat_map(|line| line?.parse::<BoardingPass>())
        .collect();
}
