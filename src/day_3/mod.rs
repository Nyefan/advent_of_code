use crate::utils::read_lines;
use tree_row::TreeRow;

pub mod part_1;
pub mod part_2;
mod tree_row;

lazy_static! {
    static ref TREE_MAP: Vec<TreeRow> = read_lines("./inputs/3.input")
        .unwrap()
        .flat_map(|line| line?.parse::<TreeRow>())
        .collect();
}

fn count(slope: &(usize, usize)) -> usize {
    TREE_MAP
        .iter()
        .step_by(slope.1)
        .enumerate()
        .filter(|&e| e.1[&(e.0 * slope.0)])
        .count()
}
