use crate::utils::read_lines;
use std::sync::Once;
use tree_row::TreeRow;

pub mod part_1;
pub mod part_2;
mod tree_row;

static mut TREE_MAP: Vec<TreeRow> = vec![];
static INIT: Once = Once::new();

fn get_tree_map() -> &'static Vec<TreeRow> {
    unsafe {
        INIT.call_once(|| {
            TREE_MAP = read_lines("./inputs/3.input")
                .unwrap()
                .flat_map(|line| line?.parse::<TreeRow>())
                .collect()
        });
        &TREE_MAP
    }
}

fn count(slope: &(usize, usize)) -> usize {
    get_tree_map()
        .iter()
        .step_by(slope.1)
        .enumerate()
        .filter(|&e| e.1[&(e.0 * slope.0)])
        .count()
}
