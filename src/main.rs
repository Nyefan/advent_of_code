use std::error::Error;

mod day_1;
mod day_2;
pub mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    day_1::part_1::main()?;
    day_1::part_2::main()?;
    day_2::part_1::main()?;
    Ok(())
}
