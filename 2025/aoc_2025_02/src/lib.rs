use crate::error::AOCError;
use nom::character::complete::{char, digit1};
use nom::combinator::{all_consuming, map_res, rest};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use std::borrow::Cow;
use std::ops::RangeInclusive;
use std::vec::IntoIter;

pub mod error;
pub mod part1;
pub mod part2;

pub type Result<'a, T> = std::result::Result<T, AOCError>;

#[derive(Clone, Debug)]
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

impl<'a> Range<'a> {
    pub fn ranges_of_constant_powers_of_10(&'_ self) -> Result<'_, IntoIter<Range<'_>>> {
        let mut ranges: Vec<Range<'_>> = vec![];
        if self.start.len() == self.end.len() {
            ranges.push(self.clone());
        } else {
            ranges.push(Range {
                start: Cow::Borrowed(&self.start),
                start_num: self.start_num,
                end: Cow::Owned("9".repeat(self.start.len())),
                end_num: "9".repeat(self.start.len()).parse::<u64>()?,
            });
            for i in self.start.len() + 1..=self.end.len() - 1 {
                ranges.push(Range {
                    start: Cow::Owned("1".to_string() + &"0".repeat(i - 1)),
                    start_num: ("1".to_string() + &"0".repeat(i - 1)).parse::<u64>()?,
                    end: Cow::Owned("9".repeat(i)),
                    end_num: "9".repeat(i).parse::<u64>()?,
                });
            }
            ranges.push(Range {
                start: Cow::Owned("1".to_string() + &"0".repeat(self.end.len() - 1)),
                start_num: ("1".to_string() + &"0".repeat(self.end.len() - 1)).parse::<u64>()?,
                end: Cow::Borrowed(&self.end),
                end_num: self.end_num,
            });
        }
        Ok(ranges.into_iter())
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
