use super::{aoc_parse, AOCError, Range, Result};
use std::borrow::Cow;

pub fn process(input: &str) -> Result<'_, String> {
    let ranges = aoc_parse(input)?.1;
    let mut invalid_ids: Vec<String> = vec![];
    for range in ranges {
        invalid_ids.append(&mut faster_invalid_ids_in_range(range)?)
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

fn naive_invalid_ids_in_range(range: Range<'_>) -> Result<'_, Vec<String>> {
    // naive version
    let mut invalid_ids: Vec<String> = vec![];
    for i in range {
        let id = i.to_string();
        if id[0..id.len() / 2] == id[id.len() / 2..] {
            invalid_ids.push(id)
        }
    }
    Ok(invalid_ids)
}

fn faster_invalid_ids_in_range(range: Range<'_>) -> Result<'_, Vec<String>> {
    fn invalid_ids_in_range_of_constant_power_of_10<'a>(
        range: &'a Range,
    ) -> Result<'a, Vec<String>> {
        dbg!(&range.start, &range.end);
        let mut invalid_ids: Vec<String> = vec![];
        if range.start.len().is_multiple_of(2) {
            for i in range.start[0..range.start.len() / 2].parse::<u64>()?
                ..=range.end[0..range.end.len() / 2].parse::<u64>()?
            {
                let id = i.to_string().repeat(2);
                let id_num = id.parse::<u64>()?;
                dbg!(&i, &id);
                if range.start_num <= id_num && id_num <= range.end_num {
                    invalid_ids.push(id)
                }
            }
        }
        Ok(invalid_ids)
    }
    let mut invalid_ids: Vec<String> = vec![];
    dbg!(&range.start, &range.end);

    if range.start.len() == range.end.len() {
        invalid_ids.append(&mut invalid_ids_in_range_of_constant_power_of_10(&range)?)
    } else {
        invalid_ids.append(&mut invalid_ids_in_range_of_constant_power_of_10(&Range {
            start: Cow::Borrowed(&range.start),
            start_num: range.start_num,
            end: Cow::Owned("9".repeat(range.start.len())),
            end_num: "9".repeat(range.start.len()).parse::<u64>()?,
        })?);
        for i in range.start.len() + 1..=range.end.len() - 1 {
            invalid_ids.append(&mut invalid_ids_in_range_of_constant_power_of_10(&Range {
                start: Cow::Owned("1".to_string() + &"0".repeat(i - 1)),
                start_num: ("1".to_string() + &"0".repeat(i - 1)).parse::<u64>()?,
                end: Cow::Owned("9".repeat(i)),
                end_num: "9".repeat(i).parse::<u64>()?,
            })?)
        }
        invalid_ids.append(&mut invalid_ids_in_range_of_constant_power_of_10(&Range {
            start: Cow::Owned("1".to_string() + &"0".repeat(range.end.len() - 1)),
            start_num: ("1".to_string() + &"0".repeat(range.end.len() - 1)).parse::<u64>()?,
            end: Cow::Borrowed(&range.end),
            end_num: range.end_num,
        })?);
    }

    dbg!(&invalid_ids);
    Ok(invalid_ids)
}
