use std::error::Error;
use std::str::FromStr;

pub mod part_1;
pub mod part_2;

struct PasswordSpecification {
    range: (i32, i32),
    letter: char,
    password: String,
}

impl FromStr for PasswordSpecification {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let mut range_iter = split
            .next()
            .unwrap()
            .split("-")
            .map(|val| val.parse::<i32>())
            .take(2);
        Ok(PasswordSpecification {
            range: (range_iter.next().unwrap()?, range_iter.next().unwrap()?),
            letter: split
                .next()
                .unwrap()
                .chars()
                .next()
                .ok_or("expected value")?,
            password: split.next().unwrap().to_string(),
        })
    }
}