use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let lines = common::parse_file(&args[1])?;
    let part_1 = part_1(&lines)?;
    let part_2 = part_2(&lines)?;
    println!("part_1: {}", part_1);
    println!("part_2: {}", part_2);
    Ok(())
}

fn part_1(lines: &[Vec<i32>]) -> Result<i32, Box<dyn Error>> {
    let safe_lines_count = lines.iter().filter(|&line| line.is_safe()).count() as i32;
    Ok(safe_lines_count)
}

fn part_2(lines: &[Vec<i32>]) -> Result<i32, Box<dyn Error>> {
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
