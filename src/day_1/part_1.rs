use crate::utils;

pub fn main() {
    let year = 2020;

    if let Ok((first_half, second_half)) = utils::bisect(year) {
        let mut index = 0;
        'outer: for first in first_half {
            'inner: for (i, second) in second_half[index..].iter().enumerate() {
                if first + second == year {
                    println!(
                        "{0} + {1} = {2}; {0} * {1} = {3}",
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
    }
}
