use super::{aoc_parse, Range, Result};
use crate::error::AOCError;
use itertools::Itertools;

pub fn process(input: &str) -> Result<'_, String> {
    let ranges = aoc_parse(input)?.1;
    let mut invalid_ids: Vec<String> = vec![];
    for range in ranges {
        invalid_ids.append(&mut invalid_ids_in_range(range)?)
    }
    let sum = invalid_ids
        .iter()
        .map(String::as_str)
        .map(str::parse::<u64>)
        .map(|r| r.map_err(AOCError::from))
        .collect::<Result<Vec<u64>>>()?
        .iter()
        .sum::<u64>();

    Ok(sum.to_string())
}

fn invalid_ids_in_range(range: Range<'_>) -> Result<'_, Vec<String>> {
    fn invalid_ids_in_range_of_constant_power_of_10<'a>(
        range: &'a Range,
    ) -> Result<'a, Vec<String>> {
        let mut invalid_ids: Vec<String> = vec![];
        for mult in 2..=range.start.len() {
            if range.start.len().is_multiple_of(mult) {
                for i in range.start[0..range.start.len() / mult].parse::<u64>()?
                    ..=range.end[0..range.end.len() / mult].parse::<u64>()?
                {
                    let id = i.to_string().repeat(mult);
                    let id_num = id.parse::<u64>()?;
                    if range.start_num <= id_num && id_num <= range.end_num {
                        invalid_ids.push(id)
                    }
                }
            }
        }
        Ok(invalid_ids.iter().unique().cloned().collect())
    }

    let mut invalid_ids: Vec<String> = vec![];
    for range in range.ranges_of_constant_powers_of_10()? {
        invalid_ids.append(&mut invalid_ids_in_range_of_constant_power_of_10(&range)?)
    }
    Ok(invalid_ids)
}

pub fn process_naive(input: &str) -> Result<'_, String> {
    let ranges = aoc_parse(input)?.1;
    let mut invalid_ids: Vec<String> = vec![];
    for range in ranges {
        invalid_ids.append(&mut invalid_ids_in_range_naive(range)?)
    }
    let sum = invalid_ids
        .iter()
        .map(String::as_str)
        .map(str::parse::<u64>)
        .map(|r| r.map_err(AOCError::from))
        .collect::<Result<Vec<u64>>>()?
        .iter()
        .sum::<u64>();

    Ok(sum.to_string())
}

fn invalid_ids_in_range_naive(range: Range<'_>) -> Result<'_, Vec<String>> {
    // naive version
    let mut invalid_ids: Vec<String> = vec![];
    for i in range {
        let id = i.to_string();
        'inner: for i in 0..id.len() / 2 {
            if is_repeated(&id[0..=i], &id) {
                invalid_ids.push(id);
                break 'inner;
            }
        }
    }
    Ok(invalid_ids)
}

fn is_repeated(test_str: &str, id: &str) -> bool {
    id.len().is_multiple_of(test_str.len()) && test_str.repeat(id.len() / test_str.len()).eq(id)
}

pub fn process_flatmap(input: &str) -> Result<'_, u64> {
    let ranges = aoc_parse(input)?.1;
    ranges
        .into_iter()
        .flat_map(invalid_ids_in_range)
        .flatten()
        .map(|s| str::parse::<u64>(&s))
        .map(|r| r.map_err(AOCError::from))
        .sum::<Result<u64>>()
}
