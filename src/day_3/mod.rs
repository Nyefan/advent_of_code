pub mod part_1;
pub mod part_2;

use std::error::Error;
use std::ops::Index;
use std::str::FromStr;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct TreeRow {
    row: Vec<bool>,
}

impl Display for TreeRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.row.iter().map(|&e| if e { '#' } else { '.' }).collect::<String>())
    }
}

impl Index<&'_ usize> for TreeRow {
    type Output = bool;

    fn index(&self, index: &usize) -> &Self::Output {
        &self.row[index % &self.row.len()]
    }
}

impl FromStr for TreeRow {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row: Vec<bool> = s
            .chars()
            .map(|c| {
                match c {
                    '.' => Ok(false),
                    '#' => Ok(true),
                    _ => Err("failed parsing file"),
                }
                .unwrap()
            })
            .collect();
        Ok(TreeRow { row })
    }
}


