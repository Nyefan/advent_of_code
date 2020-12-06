use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let year = 2020;

    if let Ok(lines) = read_lines("./inputs/1-1") {
        let (mut first_half, mut second_half) = lines
            .map(|line| {
                line.expect("failed reading file")
                    .trim()
                    .parse::<i32>()
                    .expect("not a number")
            })
            .partition::<Vec<i32>, _>(|&i| i < year / 2);
        first_half.sort();
        first_half.reverse();
        second_half.sort();
        println!("{:?}", first_half);
        println!("{:?}", second_half);
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
