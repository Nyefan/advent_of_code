type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: String) -> Result<i32> {
    let mut map = parse(input)?;
    let mut cursor = map
        .get_cursor()
        .ok_or("Error parsing map: no starting cursor")?;
    map[cursor.0][cursor.1] = Tile::Empty;
    while StepResult::Continue == step(&mut map, &mut cursor)? {}
    let result = map
        .iter()
        .flatten()
        .filter(|tile| {
            if let Tile::Visited { .. } = **tile {
                return true;
            }
            false
        })
        .count() as i32;
    Ok(result)
}

pub fn part_2(input: String) -> Result<i32> {
    let mut map = parse(input)?;
    let mut cursor = map
        .get_cursor()
        .ok_or("Error parsing map: no starting cursor")?;
    map[cursor.0][cursor.1] = Tile::Empty;
    while StepResult::Continue == step(&mut map, &mut cursor)? {}
    let result = map
        .iter()
        .flatten()
        .filter_map(|tile| {
            match tile {
                // Tile::Visited { up: true, down: true, left: true, right: true } => Some(2),
                // 
                // Tile::Visited { up: false, down: true, left: true, right: true } => Some(2),
                // Tile::Visited { up: true, down: false, left: true, right: true } => Some(2),
                // Tile::Visited { up: true, down: true, left: false, right: true } => Some(2),
                // Tile::Visited { up: true, down: true, left: true, right: false } => Some(2),

                Tile::Visited { up: false, down: false, left: true, right: true } => Some(1),
                Tile::Visited { up: false, down: true, left: false, right: true } => Some(1),
                Tile::Visited { up: false, down: true, left: true, right: false } => Some(1),
                Tile::Visited { up: true, down: false, left: false, right: true } => Some(1),
                Tile::Visited { up: true, down: false, left: true, right: false } => Some(1),
                Tile::Visited { up: true, down: true, left: false, right: false } => Some(1),

                _ => return None,
            }
        })
        .sum();
    Ok(result)
}

fn parse(input: String) -> Result<Map> {
    input
        .lines()
        .map(|line| -> Result<Vec<Tile>> {
            line.chars()
                .map(|c| -> Result<Tile> { Ok(c.try_into()?) })
                .collect()
        })
        .collect::<Result<Map>>()
}

fn step(map: &mut Map, cursor: &mut Cursor) -> Result<StepResult> {
    use crate::StepResult::{Continue, Stop, Loop};
    let horizontal_length = map[0].len();
    let vertical_length = map.len();
    let mut current_tile = &mut map[cursor.0][cursor.1];
    match cursor.2 {
        Direction::Up => match current_tile {
            Tile::Empty => {
                *current_tile = Tile::Visited {
                    up: true,
                    down: false,
                    left: false,
                    right: false,
                };
                if cursor.0 == 0 {
                    return Ok(Stop);
                }
                match map[cursor.0 - 1][cursor.1] {
                    Tile::Empty | Tile::Visited { .. } => {
                        cursor.0 -= 1;
                        Ok(Continue)
                    }
                    Tile::Obstacle => {
                        cursor.2 = Direction::Right;
                        Ok(Continue)
                    }
                    _ => Err("Invalid State Reached".into()),
                }
            }
            Tile::Visited { up: true, .. } => Ok(Loop),
            Tile::Visited {
                up: false,
                down,
                left,
                right,
            } => {
                *current_tile = Tile::Visited {
                    up: true,
                    down: *down,
                    left: *left,
                    right: *right,
                };
                if cursor.0 == 0 {
                    return Ok(Stop);
                }
                match map[cursor.0 - 1][cursor.1] {
                    Tile::Empty | Tile::Visited { .. } => {
                        cursor.0 -= 1;
                        return Ok(Continue);
                    }
                    Tile::Obstacle => {
                        cursor.2 = Direction::Right;
                        return Ok(Continue);
                    }
                    _ => Err("Invalid State Reached".into()),
                }
            }
            _ => Err("Invalid State Reached".into()),
        },
        Direction::Down => match current_tile {
            Tile::Empty => {
                *current_tile = Tile::Visited {
                    up: false,
                    down: true,
                    left: false,
                    right: false,
                };
                if cursor.0 == vertical_length - 1 {
                    return Ok(Stop);
                }
                match map[cursor.0 + 1][cursor.1] {
                    Tile::Empty | Tile::Visited { .. } => {
                        cursor.0 += 1;
                        Ok(Continue)
                    }
                    Tile::Obstacle => {
                        cursor.2 = Direction::Left;
                        Ok(Continue)
                    }
                    _ => Err("Invalid State Reached".into()),
                }
            }
            Tile::Visited { down: true, .. } => Ok(Loop),
            Tile::Visited {
                up,
                down: false,
                left,
                right,
            } => {
                *current_tile = Tile::Visited {
                    up: *up,
                    down: true,
                    left: *left,
                    right: *right,
                };
                if cursor.0 == vertical_length - 1 {
                    return Ok(Stop);
                }
                match map[cursor.0 + 1][cursor.1] {
                    Tile::Empty | Tile::Visited { .. } => {
                        cursor.0 += 1;
                        Ok(Continue)
                    }
                    Tile::Obstacle => {
                        cursor.2 = Direction::Left;
                        Ok(Continue)
                    }
                    _ => Err("Invalid State Reached".into()),
                }
            }
            _ => Err("Invalid State Reached".into()),
        },
        Direction::Left => match current_tile {
            Tile::Empty => {
                *current_tile = Tile::Visited {
                    up: false,
                    down: false,
                    left: true,
                    right: false,
                };
                if cursor.1 == 0 {
                    return Ok(Stop);
                }
                match map[cursor.0][cursor.1 - 1] {
                    Tile::Empty | Tile::Visited { .. } => {
                        cursor.1 -= 1;
                        Ok(Continue)
                    }
                    Tile::Obstacle => {
                        cursor.2 = Direction::Up;
                        Ok(Continue)
                    }
                    _ => Err("Invalid State Reached".into()),
                }
            }
            Tile::Visited { left: true, .. } => Ok(Loop),
            Tile::Visited {
                up,
                down,
                left: false,
                right,
            } => {
                *current_tile = Tile::Visited {
                    up: *up,
                    down: *down,
                    left: true,
                    right: *right,
                };
                if cursor.1 == 0 {
                    return Ok(Stop);
                }
                match map[cursor.0][cursor.1 - 1] {
                    Tile::Empty | Tile::Visited { .. } => {
                        cursor.1 -= 1;
                        Ok(Continue)
                    }
                    Tile::Obstacle => {
                        cursor.2 = Direction::Up;
                        Ok(Continue)
                    }
                    _ => Err("Invalid State Reached".into()),
                }
            }
            _ => Err("Invalid State Reached".into()),
        },
        Direction::Right => match current_tile {
            Tile::Empty => {
                *current_tile = Tile::Visited {
                    up: false,
                    down: false,
                    left: false,
                    right: true,
                };
                if cursor.0 == horizontal_length - 1 {
                    return Ok(Stop);
                }
                match map[cursor.0][cursor.1 + 1] {
                    Tile::Empty | Tile::Visited { .. } => {
                        cursor.1 += 1;
                        Ok(Continue)
                    }
                    Tile::Obstacle => {
                        cursor.2 = Direction::Down;
                        Ok(Continue)
                    }
                    _ => Err("Invalid State Reached".into()),
                }
            }
            Tile::Visited { right: true, .. } => Ok(Loop),
            Tile::Visited {
                up,
                down,
                left,
                right: false,
            } => {
                *current_tile = Tile::Visited {
                    up: *up,
                    down: *down,
                    left: *left,
                    right: true,
                };
                if cursor.0 == horizontal_length - 1 {
                    return Ok(Stop);
                }
                match map[cursor.0][cursor.1 + 1] {
                    Tile::Empty | Tile::Visited { .. } => {
                        cursor.1 += 1;
                        Ok(Continue)
                    }
                    Tile::Obstacle => {
                        cursor.2 = Direction::Down;
                        Ok(Continue)
                    }
                    _ => Err("Invalid State Reached".into()),
                }
            }
            _ => Err("Invalid State Reached".into()),
        },
    }
}

fn print_map(map: &Map) {
    println!(
        "{}",
        map.iter()
            .map(|row| { row.iter().map(String::from).collect::<String>() })
            .collect::<Vec<_>>()
            .join("\n")
    );
}

type Map = Vec<Vec<Tile>>;

type Cursor = (usize, usize, Direction);

trait GetCursor {
    fn get_cursor(&self) -> Option<Cursor>;
}

impl GetCursor for Map {
    fn get_cursor(&self) -> Option<Cursor> {
        self.iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, tile)| (i, j, tile)))
            .find(|tile| {
                matches! {tile.2, Tile::Cursor { .. }}
            })
            .map(|(i, j, tile)| match tile {
                Tile::Cursor { direction } => (i, j, *direction),
                _ => panic!("Error parsing map: cursor is not a tile"),
            })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Obstacle,
    Cursor {
        direction: Direction,
    },
    Visited {
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum StepResult {
    Continue,
    Stop,
    Loop
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Obstacle),
            'X' => Ok(Tile::Visited {
                up: false,
                down: false,
                left: false,
                right: false,
            }),
            '^' => Ok(Tile::Cursor {
                direction: Direction::Up,
            }),
            '>' => Ok(Tile::Cursor {
                direction: Direction::Right,
            }),
            'v' => Ok(Tile::Cursor {
                direction: Direction::Down,
            }),
            '<' => Ok(Tile::Cursor {
                direction: Direction::Left,
            }),
            _ => Err(format!("Error parsing tile: {} is not a valid tile", c)),
        }
    }
}

impl From<&Tile> for String {
    fn from(t: &Tile) -> Self {
        match t {
            Tile::Empty => ".".to_string(),
            Tile::Obstacle => "#".to_string(),
            Tile::Cursor {
                direction: Direction::Up,
            } => "^".to_string(),
            Tile::Cursor {
                direction: Direction::Right,
            } => ">".to_string(),
            Tile::Cursor {
                direction: Direction::Down,
            } => "v".to_string(),
            Tile::Cursor {
                direction: Direction::Left,
            } => "<".to_string(),
            Tile::Visited {
                up: _,
                down: _,
                left: _,
                right: _,
            } => "X".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let result_sample = part_1(std::fs::read_to_string("data/sample")?)?;
        assert_eq!(result_sample, 41);
        let result_actual = part_1(std::fs::read_to_string("data/actual")?)?;
        assert_eq!(result_actual, 5199);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // let result_sample = part_2(std::fs::read_to_string("data/sample")?)?;
        // assert_eq!(result_sample, 6);
        // let result_actual = part_2(std::fs::read_to_string("data/actual")?)?;
        // assert_eq!(result_actual, 4151);
        Ok(())
    }
}
