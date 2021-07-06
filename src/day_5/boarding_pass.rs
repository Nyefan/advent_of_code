use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct BoardingPass {
    pub row: usize,
    pub column: usize,
    pub seat_id: usize,
}

impl FromStr for BoardingPass {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, column) = s.split_at(7);
        let (row, column) = (str_to_num(row, "F", "B")?, str_to_num(column, "L", "R")?);
        Ok(BoardingPass {
            row,
            column,
            seat_id: row * 8 + column,
        })
    }
}

fn str_to_num(s: &str, zero: &str, one: &str) -> Result<usize, ParseIntError> {
    let s = s.replace(zero, "0").replace(one, "1");
    usize::from_str_radix(s.as_str(), 2)
}
test1