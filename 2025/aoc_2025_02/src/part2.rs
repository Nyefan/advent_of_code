use super::{aoc_parse, Range, Result};
use crate::error::AOCError;

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
    // naive version
    let mut invalid_ids: Vec<String> = vec![];
    for i in range.start.parse::<u64>()?..=range.end.parse::<u64>()? {
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
