use super::{Result, aoc_parse};
use crate::Rotation::{Left, Right};

pub fn process(input: &str) -> Result<'_, String> {
    let rotations = aoc_parse(input)?.1;
    let mut dial_position = 50;
    let mut zero_count = 0;
    for rotation in rotations {
        match rotation {
            Left(amount) => {
                dial_position -= amount;
            }
            Right(amount) => {
                dial_position += amount;
            }
        }
        dial_position = dial_position.rem_euclid(100);
        if dial_position == 0 {
            zero_count += 1;
        }
    }
    Ok(zero_count.to_string())
}
