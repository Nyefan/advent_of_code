use super::{AOCError, Range, RangeNumeric, Result, aoc_parse, aoc_parse_numeric};

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
        if range.start.len().is_multiple_of(2) {
            for i in range.start[0..range.start.len() / 2].parse::<u64>()?
                ..=range.end[0..range.end.len() / 2].parse::<u64>()?
            {
                let id = i.to_string().repeat(2);
                let id_num = id.parse::<u64>()?;
                if range.start_num <= id_num && id_num <= range.end_num {
                    invalid_ids.push(id)
                }
            }
        }
        Ok(invalid_ids)
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
        if id[0..id.len() / 2] == id[id.len() / 2..] {
            invalid_ids.push(id)
        }
    }
    Ok(invalid_ids)
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

pub fn process_flatmap_numeric(input: &str) -> Result<'_, u64> {
    let sum = aoc_parse_numeric(input)?
        .1
        .into_iter()
        .flat_map(RangeNumeric::ranges_of_constant_powers_of_10)
        .flat_map(invalid_ids_in_range_numeric_of_constant_power_of_10)
        .sum::<u64>();
    Ok(sum)
}

fn invalid_ids_in_range_numeric_of_constant_power_of_10(range: RangeNumeric) -> Vec<u64> {
    #[inline(always)]
    fn rshift_base10(i: u64, shift: u32) -> u64 {
        i / (10_u64.pow(shift))
    }
    #[inline(always)]
    fn repeat(i: u64, digits: u32, repetitions: u32) -> u64 {
        let mut res = i;
        for _ in 1..repetitions {
            res *= 10_u64.pow(digits);
            res += i;
        }
        res
    }
    #[inline(always)]
    fn digits(i: u64) -> u32 {
        i.ilog10() + 1
    }

    let mut invalid_ids: Vec<u64> = vec![];
    let digits = digits(range.start);
    let high_start = rshift_base10(range.start, digits / 2);
    let high_end = rshift_base10(range.end, digits / 2);
    if digits.is_multiple_of(2) {
        for i in high_start..=high_end {
            let id = repeat(i, digits / 2, 2);
            if range.start <= id && id <= range.end {
                invalid_ids.push(id)
            }
        }
    }
    invalid_ids
}
