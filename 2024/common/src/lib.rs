use std::io;
use std::str::FromStr;

pub fn parse_file<T: FromStr>(path: &str) -> io::Result<Vec<Vec<T>>>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<T>().unwrap())
                .collect::<Vec<T>>()
        })
        .collect::<Vec<Vec<T>>>())
}
