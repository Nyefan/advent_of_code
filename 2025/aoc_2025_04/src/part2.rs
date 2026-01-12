use super::{Floor, Result};
use itertools::Itertools;

pub fn process(input: &str) -> Result<'_, u32> {
    let mut floor = Floor::from(input);
    let mut removed_count = 0;
    let mut removed_flag = true;
    while removed_flag {
        removed_flag = false;
        let removed_tiles = floor
            .iter()
            .filter(|tile| tile.is_roll())
            .filter(|tile| floor.get_surrounding_roll_count(&tile) < 4)
            .cloned()
            .collect::<Vec<_>>();
        for tile in removed_tiles {
            floor.remove_roll(&tile);
            removed_flag = true;
            removed_count += 1;
        }
    }
    Ok(removed_count)
}

pub fn process_fast_sliding_window(input: &str) -> Result<'_, u32> {
    let mut floor = Floor::from(input);
    let mut forward_index;
    let mut trailing_index = 0;
    let mut removed_tiles = floor
        .iter()
        .filter(|&tile| tile.is_roll())
        .filter(|&tile| floor.get_surrounding_roll_count(tile) < 4)
        .cloned()
        .collect::<Vec<_>>();
    forward_index = removed_tiles.len();
    while forward_index != trailing_index {
        removed_tiles[trailing_index..forward_index]
            .iter()
            .for_each(|&tile| floor.remove_roll(&tile));
        let mut new_removed_tiles = removed_tiles[trailing_index..forward_index]
            .iter()
            .flat_map(|&tile| floor.get_surrounding_tiles(&tile))
            .flatten()
            .filter(|&tile| tile.is_roll())
            .filter(|&tile| floor.get_surrounding_roll_count(tile) < 4)
            .unique()
            .cloned()
            .collect::<Vec<_>>();
        removed_tiles.append(&mut new_removed_tiles);
        trailing_index = forward_index;
        forward_index = removed_tiles.len();
    }

    Ok(removed_tiles.len() as u32)
}
