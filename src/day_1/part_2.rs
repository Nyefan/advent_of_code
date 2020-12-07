use crate::utils;
use std::io::Error;

pub fn main() -> Result<(), Error> {
    let year = 2020;

    let (first_half, second_half) = utils::bisect(year)?;
    let mut second_half_index = 1;
    'first: for (i, first) in first_half.iter().enumerate() {
        for (j, second) in first_half[i + 1..].iter().enumerate() {
            if first + second > year / 2 {
                'thirdif: for third in first_half[j + 1..].iter() {
                    if first + second + third == year {
                        print_result(first, second, third);
                        break 'first;
                    }
                    if first + second + third < year {
                        break 'thirdif;
                    }
                }
            } else {
                'thirdelse: for (k, third) in second_half[second_half_index..].iter().enumerate() {
                    if first + second + third == year {
                        print_result(first, second, third);
                        break 'first;
                    }
                    if first + second + third > year {
                        second_half_index = k;
                        break 'thirdelse;
                    }
                }
            }
        }
    }
    Ok(())
}

fn print_result(first: &i32, second: &i32, third: &i32) {
    println!(
        "day_01::part_2:\t {0} + {1} + {2} = {3}; {0} * {1} * {2} = {4}",
        first,
        second,
        third,
        first + second + third,
        first * second * third
    );
}
