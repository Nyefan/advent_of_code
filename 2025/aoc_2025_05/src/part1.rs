use super::Result;
use crate::error::{AOCError, ParseErrors};
use itertools::Itertools;

pub fn process(input: &str) -> Result<'_, u32> {
    dbg!(&input);
    let (ranges, ingredients) = input
        .split("\n\n")
        .collect_tuple()
        .ok_or(AOCError::ParseError(ParseErrors::InputFormatError()))?;
    let ranges = RangesNaive::parse_naive(ranges)?;
    let fresh_ingredient_count = ingredients
        .lines()
        .map(|line| line.parse::<u64>())
        .filter_ok(|&num| ranges.any_range_includes(num))
        .count();
    Ok(fresh_ingredient_count as u32)
}

#[derive(Debug)]
struct RangesNaive {
    ranges: Vec<(u64, u64)>,
}

impl RangesNaive {
    fn parse_naive(input: &str) -> Result<'_, Self> {
        let ranges: Vec<(u64, u64)> = input
            .lines()
            .map(|line| {
                line.split('-')
                    .map(|num| num.parse::<u64>())
                    .collect_tuple()
                    .ok_or(AOCError::ParseError(ParseErrors::InputFormatError()))
                    .map(|(start, end)| -> Result<(u64, u64)> { Ok((start?, end?)) })?
            })
            .collect::<Result<Vec<(u64, u64)>>>()?;
        Ok(Self { ranges })
    }

    fn any_range_includes(&self, num: u64) -> bool {
        self.ranges
            .iter()
            .any(|range| range.0 <= num && num <= range.1)
    }
}
