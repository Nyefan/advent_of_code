use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

pub fn bisect(year: i32) -> Result<(Vec<i32>, Vec<i32>), Error> {
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

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
