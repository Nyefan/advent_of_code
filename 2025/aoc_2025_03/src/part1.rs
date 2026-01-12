use super::Result;

pub fn process(input: &str) -> Result<'_, String> {
    let total_joltage = input.lines().map(find_max_joltage_for_line).sum::<u32>();
    Ok(total_joltage.to_string())
}

fn find_max_joltage_for_line(line: &str) -> u32 {
    let mut first_digit = '0';
    let mut second_digit = '0';
    let mut iter = line.chars().peekable();
    while let Some(character) = iter.next() {
        if character > first_digit
            && let Some(&next) = iter.peek()
        {
            first_digit = character;
            second_digit = next;
        } else if character > second_digit {
            second_digit = character;
        }
    }
    10 * first_digit.to_digit(10).unwrap() + second_digit.to_digit(10).unwrap()
}
