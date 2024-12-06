type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &[&[u8]]) -> Result<i32> {
    let result = input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(y, char)| (i, y, char)))
        .flat_map(|(i, j, char)| {
            let check = |i: usize, j: usize, c: u8| -> bool {
                input
                    .get(i)
                    .and_then(|&line| line.get(j))
                    .is_some_and(|v| *v == c)
            };
            if *char != b'X' {
                return [false; 8];
            }

            [
                //right
                check(i + 1, j, b'M') && check(i + 2, j, b'A') && check(i + 3, j, b'S'),
                //down-right
                check(i + 1, j + 1, b'M') && check(i + 2, j + 2, b'A') && check(i + 3, j + 3, b'S'),
                //down
                check(i, j + 1, b'M') && check(i, j + 2, b'A') && check(i, j + 3, b'S'),
                //down-left
                j >= 3
                    && check(i + 1, j - 1, b'M')
                    && check(i + 2, j - 2, b'A')
                    && check(i + 3, j - 3, b'S'),
                //left
                j >= 3 && check(i, j - 1, b'M') && check(i, j - 2, b'A') && check(i, j - 3, b'S'),
                //up-left
                i >= 3
                    && j >= 3
                    && check(i - 1, j - 1, b'M')
                    && check(i - 2, j - 2, b'A')
                    && check(i - 3, j - 3, b'S'),
                //up
                i >= 3 && check(i - 1, j, b'M') && check(i - 2, j, b'A') && check(i - 3, j, b'S'),
                //up-right
                i >= 3
                    && check(i - 1, j + 1, b'M')
                    && check(i - 2, j + 2, b'A')
                    && check(i - 3, j + 3, b'S'),
            ]
        })
        .filter(|b| *b)
        .count() as i32;
    Ok(result)
}

pub fn part_2(input: &[&[u8]]) -> Result<i32> {
    let result = input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(y, char)| (i, y, char)))
        .flat_map(|(i, j, char)| {
            let check = |i: usize, j: usize, c: u8| -> bool {
                input
                    .get(i)
                    .and_then(|&line| line.get(j))
                    .is_some_and(|v| *v == c)
            };
            if *char != b'A' || i < 1 || j < 1 {
                return [false; 4];
            }

            [
                check(i - 1, j - 1, b'M')
                    && check(i + 1, j + 1, b'S')
                    && check(i - 1, j + 1, b'M')
                    && check(i + 1, j - 1, b'S'),
                check(i - 1, j - 1, b'M')
                    && check(i + 1, j + 1, b'S')
                    && check(i + 1, j - 1, b'M')
                    && check(i - 1, j + 1, b'S'),
                check(i + 1, j + 1, b'M')
                    && check(i - 1, j - 1, b'S')
                    && check(i - 1, j + 1, b'M')
                    && check(i + 1, j - 1, b'S'),
                check(i + 1, j + 1, b'M')
                    && check(i - 1, j - 1, b'S')
                    && check(i + 1, j - 1, b'M')
                    && check(i - 1, j + 1, b'S'),
            ]
        })
        .filter(|b| *b)
        .count() as i32;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let binding_sample = common::parse_lines("data/sample")?;
        let lines_sample: Vec<&[u8]> = binding_sample
            .iter()
            .map(|s: &String| s.as_bytes())
            .collect();
        let result_sample = part_1(&lines_sample)?;
        assert_eq!(result_sample, 18);
        let binding_actual = common::parse_lines("data/actual")?;
        let lines_actual: Vec<&[u8]> = binding_actual
            .iter()
            .map(|s: &String| s.as_bytes())
            .collect();
        let result_actual = part_1(&lines_actual)?;
        assert_eq!(result_actual, 2336);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let binding_sample = common::parse_lines("data/sample")?;
        let lines_sample: Vec<&[u8]> = binding_sample
            .iter()
            .map(|s: &String| s.as_bytes())
            .collect();
        let result_sample = part_2(&lines_sample)?;
        assert_eq!(result_sample, 9);
        let binding_actual = common::parse_lines("data/actual")?;
        let lines_actual: Vec<&[u8]> = binding_actual
            .iter()
            .map(|s: &String| s.as_bytes())
            .collect();
        let result_actual = part_2(&lines_actual)?;
        assert_eq!(result_actual, 1831);
        Ok(())
    }
}
