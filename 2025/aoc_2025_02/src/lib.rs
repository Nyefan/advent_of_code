use crate::error::AOCError;
use nom::character::complete::{char, digit1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

pub mod error;
pub mod part1;
pub mod part2;

pub type Result<'a, T> = std::result::Result<T, AOCError>;

#[derive(Debug)]
struct Range<'a> {
    start: &'a str,
    end: &'a str,
}

fn aoc_parse(input: &str) -> IResult<&str, Vec<Range<'_>>> {
    fn range(input: &str) -> IResult<&str, Range<'_>> {
        let (input, (start, end)) = separated_pair(digit1, char('-'), digit1).parse(input)?;

        let range = Range { start, end };
        Ok((input, range))
    }
    separated_list1(char(','), range).parse(input)
}
