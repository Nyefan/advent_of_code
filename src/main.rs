use std::io::Error;

mod day_1;
pub mod utils;

fn main() -> Result<(), Error> {
    day_1::part_1::main()?;
    day_1::part_2::main()?;
    Ok(())
}
