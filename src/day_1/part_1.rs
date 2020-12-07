use std::io::Error;

pub fn main() -> Result<(), Error> {
    let year = 2020;

    let (first_half, second_half) = super::bisect(year)?;
    let mut index = 0;
    'outer: for first in first_half {
        'inner: for (i, second) in second_half[index..].iter().enumerate() {
            if first + second == year {
                println!(
                    "day_01::part_1:\t {0} + {1} = {2}; {0} * {1} = {3}",
                    first,
                    second,
                    year,
                    first * second
                );
                break 'outer;
            }
            if first + second > year {
                index = i;
                break 'inner;
            }
        }
    }
    Ok(())
}
