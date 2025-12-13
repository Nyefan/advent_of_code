use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i32, line_ending};
use nom::multi::separated_list1;
use nom::{IResult, Parser};

pub mod part1;
pub mod part2;

pub type Result<'a, T> = std::result::Result<T, Box<dyn std::error::Error + 'a>>;

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn aoc_parse(input: &str) -> IResult<&str, Vec<Rotation>> {
    fn direction(input: &str) -> IResult<&str, Rotation> {
        let (input, direction) = alt((tag("L"), tag("R"))).parse(input)?;
        let (input, amount) = i32(input)?;

        let rotation = match direction {
            "L" => Rotation::Left(amount),
            "R" => Rotation::Right(amount),
            unknown => panic!("invalid direction: {unknown}"),
        };
        Ok((input, rotation))
    }
    separated_list1(line_ending, direction).parse(input)
}
