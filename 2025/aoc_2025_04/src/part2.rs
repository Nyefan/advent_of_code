use super::{Floor, Result};

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
