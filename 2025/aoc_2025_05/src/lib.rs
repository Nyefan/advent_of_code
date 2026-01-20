use crate::error::AOCError;

pub mod error;
pub mod part1;
pub mod part2;

pub type Result<'a, T> = std::result::Result<T, AOCError>;

