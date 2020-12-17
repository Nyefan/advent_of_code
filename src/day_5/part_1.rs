use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let highest = super::BOARDINGPASSES
        .iter()
        .max_by_key(|b| b.seat_id)
        .unwrap()
        .seat_id;
    println!("day_05::part_1:\tHighest seat_id: {}", highest);
    Ok(())
}
