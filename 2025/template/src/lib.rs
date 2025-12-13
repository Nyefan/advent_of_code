pub mod part1;
pub mod part2;

pub type Result<'a, T> = std::result::Result<T, Box<dyn std::error::Error + 'a>>;
