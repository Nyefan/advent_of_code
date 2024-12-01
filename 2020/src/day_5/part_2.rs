use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut sorted = super::BOARDINGPASSES.clone();
    sorted.sort_unstable_by_key(|b| b.seat_id);
    let mut seat_id = sorted
        .iter()
        .enumerate()
        .skip_while(|e| e.0 + sorted[0].seat_id == e.1.seat_id)
        .next()
        .unwrap()
        .0;
    seat_id += sorted[0].seat_id;
    println!("day_05::part_2:\tMy seat_id: {}", seat_id);
    Ok(())
}
