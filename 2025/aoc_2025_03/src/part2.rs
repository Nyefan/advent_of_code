use super::Result;
use nom::{AsBytes, ParseTo};

pub fn process(input: &str) -> Result<'_, String> {
    let total_joltage = input.lines().map(find_max_joltage_for_line).sum::<u64>();
    Ok(total_joltage.to_string())
}

pub fn find_max_joltage_for_line(line: &str) -> u64 {
    let mut digits = [b'0'; 12];
    let bytes = line.as_bytes();
    let mut i_pivot = 0;

    for i_digit in 0..12 {
        let (mut i_line_max, mut max) = (i_pivot, bytes[i_pivot]);
        for i_line in i_pivot + 1..=bytes.len() - 12 + i_digit {
            if bytes[i_line] > max {
                (i_line_max, max) = (i_line, bytes[i_line]);
            }
        }
        digits[i_digit] = max;
        i_pivot = i_line_max + 1;
    }

    // we know this is exactly 12 digits which always fits in a u64
    digits.as_bytes().parse_to().unwrap()
}
