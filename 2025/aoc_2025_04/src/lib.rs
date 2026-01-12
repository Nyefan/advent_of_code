use crate::error::AOCError;
use std::fmt::{Display, Formatter};

pub mod error;
pub mod part1;
pub mod part2;

pub type Result<'a, T> = std::result::Result<T, AOCError>;

#[derive(Debug)]
pub struct Floor {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum TileState {
    Empty,
    Roll,
}

impl From<char> for TileState {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '@' => Self::Roll,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct TileLocation {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tile {
    state: TileState,
    location: TileLocation,
}

impl Tile {
    fn is_roll(&self) -> bool {
        self.state == TileState::Roll
    }
}

impl From<&str> for Floor {
    fn from(value: &str) -> Self {
        Floor {
            tiles: value
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| Tile {
                            state: TileState::from(c),
                            location: TileLocation { x, y },
                        })
                        .collect::<Vec<Tile>>()
                })
                .collect::<Vec<Vec<Tile>>>(),
        }
    }
}

impl Floor {
    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter().flatten()
    }
    pub fn get_surrounding_tiles(&self, tile: &Tile) -> [Option<&Tile>; 8] {
        fn try_sub(a: usize, b: usize) -> Option<usize> {
            let res = a.overflowing_sub(b);
            if res.1 { None } else { Some(res.0) }
        }
        [
            self.get_tile_at_location(try_sub(tile.location.x, 1), try_sub(tile.location.y, 1)),
            self.get_tile_at_location(try_sub(tile.location.x, 1), Some(tile.location.y)),
            self.get_tile_at_location(try_sub(tile.location.x, 1), Some(tile.location.y + 1)),
            self.get_tile_at_location(Some(tile.location.x), try_sub(tile.location.y, 1)),
            self.get_tile_at_location(Some(tile.location.x), Some(tile.location.y + 1)),
            self.get_tile_at_location(Some(tile.location.x + 1), try_sub(tile.location.y, 1)),
            self.get_tile_at_location(Some(tile.location.x + 1), Some(tile.location.y)),
            self.get_tile_at_location(Some(tile.location.x + 1), Some(tile.location.y + 1)),
        ]
    }

    fn get_tile_at_location(&self, x: Option<usize>, y: Option<usize>) -> Option<&Tile> {
        self.tiles.get(y?)?.get(x?)
    }

    pub fn get_surrounding_roll_count(&self, tile: &Tile) -> usize {
        self.get_surrounding_tiles(tile)
            .iter()
            .flatten()
            .filter(|&tile| tile.state == TileState::Roll)
            .count()
    }

    pub fn remove_roll(&mut self, tile: &Tile) {
        self.tiles[tile.location.y][tile.location.x].state = TileState::Empty;
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile.state {
                        TileState::Empty => ".",
                        TileState::Roll => "@",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
