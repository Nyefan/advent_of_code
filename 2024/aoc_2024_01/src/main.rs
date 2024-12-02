use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
    let sum = part_2()?;

    println!("Sum: {}", sum);

    Ok(())
}

fn part_1() -> io::Result<i32> {
    let mut lists: (Vec<i32>, Vec<i32>) = read_to_string("data/actual")?
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
    let sum = lists.0.iter().zip(lists.1.iter())
        .map(|(x, y)| (y - x).abs())
        .sum::<i32>();
    Ok(sum)
}

fn part_1_fast() -> io::Result<i32> {
    Ok(read_to_string("data/actual")?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .fold(0, |x, y| y - x)
        })
        .sum::<i32>()
        .abs())
}

fn part_2() -> io::Result<i32> {
    let mut lists: Vec<(i32, i32)> = read_to_string("data/actual")?
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
    let sum = counters.0.iter().map(
        |(k, v)| {
            k*v*counters.1.get(k).unwrap_or(&default_value)
        }
    ).sum();
    Ok(sum)
}