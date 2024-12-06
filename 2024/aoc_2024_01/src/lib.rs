use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn part_1(path_to_data: &str) -> Result<i32> {
    let mut lists: (Vec<i32>, Vec<i32>) = read_to_string(path_to_data)?
        .lines()
        .map(|line| {
            let line: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            (line[0], line[1])
        })
        .collect::<Vec<(i32, i32)>>()
        .into_iter()
        .unzip();
    lists.0.sort();
    lists.1.sort();
    let sum = lists
        .0
        .iter()
        .zip(lists.1.iter())
        .map(|(x, y)| (y - x).abs())
        .sum::<i32>();
    Ok(sum)
}

pub fn part_1_fast(path_to_data: &str) -> Result<i32> {
    Ok(read_to_string(path_to_data)?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .fold(0, |x, y| y - x)
        })
        .sum::<i32>()
        .abs())
}

pub fn part_2(path_to_data: &str) -> Result<i32> {
    let lists: Vec<(i32, i32)> = read_to_string(path_to_data)?
        .lines()
        .map(|line| {
            let line: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            (line[0], line[1])
        })
        .collect::<Vec<_>>();
    let mut counters: (HashMap<i32, i32>, HashMap<i32, i32>) = (HashMap::new(), HashMap::new());
    for (x, y) in lists {
        *counters.0.entry(x).or_insert(0) += 1;
        *counters.1.entry(y).or_insert(0) += 1;
    }
    let default_value = 0;
    let sum = counters
        .0
        .iter()
        .map(|(k, v)| k * v * counters.1.get(k).unwrap_or(&default_value))
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let result_sample = part_1("data/sample")?;
        assert_eq!(result_sample, 11);
        let result_actual = part_1("data/actual")?;
        assert_eq!(result_actual, 1320851);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let result_sample = part_2("data/sample")?;
        assert_eq!(result_sample, 31);
        let result_actual = part_2("data/actual")?;
        assert_eq!(result_actual, 26859182);
        Ok(())
    }
}
