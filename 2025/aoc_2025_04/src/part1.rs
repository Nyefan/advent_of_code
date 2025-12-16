use super::{Floor, Result, TileState};
use tracing::info;

pub fn process(input: &str) -> Result<'_, usize> {
    let floor = Floor::from(input);
    let result = floor
        .iter()
        .filter(|&tile| tile.state == TileState::Roll)
        .map(|tile| {
            let surrounding = floor.get_surrounding_tiles(tile);
            info!("{:?}", surrounding);
            let count = surrounding
                .into_iter()
                .flatten()
                .filter(|&tile| tile.state == TileState::Roll)
                .count();
            info!("{}", &count);
            count
        })
        .filter(|&count| count < 4)
        .count();
    Ok(result)
}
