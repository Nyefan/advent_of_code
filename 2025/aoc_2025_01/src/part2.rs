use super::{Result, aoc_parse};
use crate::Rotation::{Left, Right};

pub fn process(input: &str) -> Result<'_, String> {
    let rotations = aoc_parse(input)?.1;
    let mut dial_position = 50;
    let mut zero_count = 0;
    let mut is_previous_position_zero = false;
    for rotation in rotations {
        match rotation {
            Left(amount) => {
                zero_count += (amount) / 100;
                dial_position -= amount.rem_euclid(100);
            }
            Right(amount) => {
                zero_count += amount / 100;
                dial_position += amount.rem_euclid(100);
            }
        }
        match dial_position {
            _ if dial_position == 0 => zero_count += (!is_previous_position_zero) as i32,
            dial_position if dial_position > 0 => {
                zero_count += dial_position / 100;
            }
            dial_position if dial_position < 0 => {
                zero_count += (-dial_position) / 100 + (!is_previous_position_zero) as i32;
            }
            unknown => panic!("unexpected dial position: {unknown}"),
        }
        dial_position = dial_position.rem_euclid(100);
        is_previous_position_zero = dial_position == 0;
    }
    // we've measured all the transits above, now we just need to add one extra if the final rotation puts us at zero

    Ok(zero_count.to_string())
}
