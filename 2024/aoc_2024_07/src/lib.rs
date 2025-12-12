use itertools::Itertools;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: String) -> Result<i32> {
    Ok(0)
}

pub fn part_2(input: String) -> Result<i32> {
    Ok(0)
}

impl TryFrom<String> for Equation {
    type Error = Box<dyn std::error::Error>;
    fn try_from(input: String) -> std::result::Result<Self, Self::Error> {
        let tokens = input.split_whitespace().collect::<Vec<_>>();
        Ok(Equation {
            result: tokens[0].trim_matches(':').parse()?,
            parameters: tokens[1..]
                .iter()
                .map(|s| -> Result<i32> { Ok(s.parse::<i32>()?) })
                .collect::<Result<Vec<i32>>>()?,
        })
    }
}

fn permute(n: usize) -> Vec<Vec<Operator>> {
    [Operator::Add, Operator::Multiply]
        .iter()
        .combinations(n)
        .map(|c| {
            c.iter()
                .permutations(n)
                .map(|p| p.iter().map(|&&&o| o).collect::<Vec<Operator>>())
                .collect::<Vec<Vec<Operator>>>()
        }).flatten()
        .collect::<Vec<Vec<Operator>>>()
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Multiply,
}

struct Equation {
    result: i32,
    parameters: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let result_sample = part_1(std::fs::read_to_string("data/sample")?)?;
        assert_eq!(result_sample, 3749);
        // let result_actual = part_1(std::fs::read_to_string("data/actual")?)?;
        // assert_eq!(result_actual, 5199);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // let result_sample = part_2(std::fs::read_to_string("data/sample")?)?;
        // assert_eq!(result_sample, 6);
        // let result_actual = part_2(std::fs::read_to_string("data/actual")?)?;
        // assert_eq!(result_actual, 1915);
        Ok(())
    }
}
