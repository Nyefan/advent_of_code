use std::io::Error;
use crate::utils::read_lines;

pub mod part_1;
pub mod part_2;

fn bisect(year: i32) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let lines = read_lines("./inputs/1.input")?;
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
    Ok((first_half, second_half))
}
