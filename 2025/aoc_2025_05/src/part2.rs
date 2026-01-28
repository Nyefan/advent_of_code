use std::cmp::Ordering;
use super::Result;
use crate::error::AOCError;
use crate::error::ParseErrors::InputFormatError;
use itertools::Itertools;
use std::iter::once;
use AOCError::ParseError;
use RangeLimit::*;

pub fn process(input: &str) -> Result<u64> {
    let ranges = Ranges::parse(
        input
            .split("\n\n")
            .next()
            .ok_or(ParseError(InputFormatError()))?,
    )?
    .ranges
    .iter()
    .map(|(start, end)| match (start, end) {
        (RangeStart(start), RangeEnd(end)) if start <= end => end - start + 1,
        _ => panic!("Should not happen"),
    })
    .sum();
    Ok(ranges)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RangeLimit {
    RangeStart(u64),
    RangeEnd(u64),
}

impl Ord for RangeLimit {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (RangeStart(a), RangeStart(b)) => a.cmp(b),
            (RangeStart(a), RangeEnd(b)) => a.cmp(b),
            (RangeEnd(a), RangeStart(b)) => a.cmp(b),
            (RangeEnd(a), RangeEnd(b)) => a.cmp(b),
        }
    }
}
impl PartialOrd for RangeLimit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Ranges {
    ranges: Vec<(RangeLimit, RangeLimit)>,
}

impl Ranges {
    fn parse(input: &str) -> Result<Self> {
        let mut ranges: Vec<RangeLimit> = input
            .lines()
            .flat_map(parse_line)
            .flat_map(|tuple| once(tuple.0).chain(once(tuple.1)))
            .sorted()
            .collect();

        dbg!(&ranges);

        let mut ranges_to_delete: Vec<usize> = Vec::new();
        let mut in_range = false;
        for (i, range) in ranges.iter().enumerate().rev() {
            match (range, in_range) {
                (RangeEnd(_), false) => in_range = true,
                (RangeEnd(_), true) => ranges_to_delete.push(i),
                (RangeStart(_), true) => in_range = false,
                (RangeStart(_), false) => continue,
            }
        }
        for i in ranges_to_delete.iter() {
            ranges.remove(*i);
        }

        dbg!(&ranges);

        let mut ranges_to_delete: Vec<usize> = Vec::new();
        let mut in_range = false;
        for (i, range) in ranges.iter().enumerate() {
            match (range, in_range) {
                (RangeStart(_), false) => in_range = true,
                (RangeStart(_), true) => ranges_to_delete.push(i),
                (RangeEnd(_), true) => in_range = false,
                (RangeEnd(_), false) => panic!("Should not happen"),
            }
        }
        for i in ranges_to_delete.iter().rev() {
            ranges.remove(*i);
        }

        dbg!(&ranges);

        let ranges: Vec<(RangeLimit, RangeLimit)> = ranges
            .iter()
            .copied()
            .tuples()
            .map(|(start, end)| match (start, end) {
                (RangeStart(_), RangeEnd(_)) => (start, end),
                _ => panic!("Should not happen"),
            })
            .collect();

        dbg!(&ranges);

        return Ok(Self { ranges });

        fn parse_line(line: &str) -> Result<'_, (RangeLimit, RangeLimit)> {
            line.split('-')
                .flat_map(str::parse::<u64>)
                .collect_tuple::<(u64, u64)>()
                .ok_or(ParseError(InputFormatError()))
                .map(|(start, end)| Ok((RangeStart(start), RangeEnd(end))))?
        }
    }
}
