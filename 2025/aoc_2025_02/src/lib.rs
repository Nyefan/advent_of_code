use crate::error::AOCError;
use nom::character::complete::{char, digit1};
use nom::combinator::{all_consuming, map_res, rest};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use std::borrow::Cow;
use std::ops::RangeInclusive;

pub mod error;
pub mod part1;
pub mod part2;

pub type Result<'a, T> = std::result::Result<T, AOCError>;

#[derive(Debug)]
struct Range<'a> {
    start: Cow<'a, str>,
    end: Cow<'a, str>,
    start_num: u64,
    end_num: u64,
}

impl IntoIterator for Range<'_> {
    type Item = u64;
    type IntoIter = RangeInclusive<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.start_num..=self.end_num
    }
}

fn aoc_parse(input: &str) -> IResult<&str, Vec<Range<'_>>> {
    fn range(input: &str) -> IResult<&str, Range<'_>> {
        let (input, (start, end)) = separated_pair(digit1, char('-'), digit1).parse(input)?;
        let start_num = map_res(rest, str::parse::<u64>).parse(start)?.1;
        let end_num = map_res(rest, str::parse::<u64>).parse(end)?.1;

        let range = Range {
            start: Cow::Borrowed(start),
            end: Cow::Borrowed(end),
            start_num,
            end_num,
        };
        Ok((input, range))
    }
    all_consuming(separated_list1(char(','), range)).parse(input)
}
