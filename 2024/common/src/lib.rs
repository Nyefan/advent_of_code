use std::io;
pub fn parse_file(path: &str) -> io::Result<Vec<Vec<i32>>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|str| str.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect())
}
