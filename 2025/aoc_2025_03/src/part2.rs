use super::Result;
use nom::AsBytes;

pub fn process(input: &str) -> Result<'_, String> {
    let total_joltage = input.lines().map(find_max_joltage_for_line).sum::<u64>();
    Ok(total_joltage.to_string())
}

pub fn find_max_joltage_for_line(line: &str) -> u64 {
    let mut digits = [b'0'; 12];
    let bytes = line.as_bytes();
    let mut i_pivot = 0;

    // for (i_digit, digit) in digits.iter_mut().enumerate() {
    //     for (i_line, &byte) in bytes[0..=bytes.len() - 12 + i_digit]
    //         .iter()
    //         .enumerate()
    //         .skip(i_pivot)
    //     {
    //         if byte > *digit {
    //             (i_pivot, *digit) = (i_line + 1, byte)
    //         }
    //     }
    // }
    //
    // this is 20% faster than the equivalent for loop above
    //   because pipelining magic?
    //   it unrolled the loop in both cases and the fire graph was inconclusive
    digits.iter_mut().enumerate().for_each(|(i_digit, digit)| {
        bytes
            .iter()
            .enumerate()
            .take(bytes.len() - 12 + i_digit + 1)
            .skip(i_pivot)
            .for_each(|(i_line, &byte)| {
                if byte > *digit {
                    (i_pivot, *digit) = (i_line + 1, byte)
                }
            })
    });

    // we know this is exactly 12 ascii digits which
    // 1. is always utf8
    // 2. always fits in a u64
    String::from_utf8_lossy(&digits).parse::<u64>().unwrap()
}
