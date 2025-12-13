use super::{aoc_parse, AOCError, Range, Result};

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
        if id[0..id.len() / 2] == id[id.len() / 2..] {
            invalid_ids.push(id)
        }
    }
    Ok(invalid_ids)
    // if range.start.len() == range.end.len() {
    //     if range.start.len() % 2 != 0 {
    //         Ok(vec![])
    //     } else {
    //         let dup_range = (range.start[0..range.start.len() / 2].parse::<u64>()?)
    //             ..(range.end[0..range.start.len() / 2].parse::<u64>()?);
    //         let first_dup_range_floor = range.start[range.start.len() / 2..].parse::<u64>()?;
    //         let final_dup_range_ceiling = range.end[range.end.len() / 2..].parse::<u64>()?;
    //
    //     }
    // } else {
    // }
}
