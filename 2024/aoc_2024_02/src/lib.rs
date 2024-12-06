use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn part_1(lines: &[Vec<i32>]) -> Result<i32> {
    let safe_lines_count = lines.iter().filter(|&line| line.is_safe()).count() as i32;
    Ok(safe_lines_count)
}

pub fn part_2(lines: &[Vec<i32>]) -> Result<i32> {
    let safe_lines_count = lines
        .iter()
        .filter(|&line| line.is_safe() || line.is_safe_with_damper())
        .count() as i32;
    Ok(safe_lines_count)
}

trait IsSafe {
    fn is_safe(&self) -> bool;
    fn is_safe_with_damper(&self) -> bool;
}

impl IsSafe for &Vec<i32> {
    fn is_safe(&self) -> bool {
        let diffs = self.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
        diffs.iter().all(|x| *x >= 1 && *x <= 3) || diffs.iter().all(|x| *x <= -1 && *x >= -3)
    }

    fn is_safe_with_damper(&self) -> bool {
        for i in 0..self.len() {
            if (&self[..i]
                .iter()
                .chain(self[i + 1..].iter())
                .cloned()
                .collect::<Vec<i32>>())
                .is_safe()
            {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let lines_sample = common::parse_file("data/sample")?;
        let result_sample = part_1(&lines_sample)?;
        assert_eq!(result_sample, 2);
        let lines_actual = common::parse_file("data/actual")?;
        let result_actual = part_1(&lines_actual)?;
        assert_eq!(result_actual, 432);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let lines_sample = common::parse_file("data/sample")?;
        let result_sample = part_2(&lines_sample)?;
        assert_eq!(result_sample, 4);
        let lines_actual = common::parse_file("data/actual")?;
        let result_actual = part_2(&lines_actual)?;
        assert_eq!(result_actual, 488);
        Ok(())
    }
}
