use crate::day_4::passport::Passport;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let count = super::PASSPORTS.iter().filter(validator).count();
    println!("day_04::part_2:\tValid passport count: {}", count);
    Ok(())
}

fn validator(p: &&Passport) -> bool {
    p.birth_year.as_ref()
        .and(p.issue_year.as_ref())
        .and(p.expiration_year.as_ref())
        .and(p.height.as_ref())
        .and(p.hair_color.as_ref())
        .and(p.eye_color.as_ref())
        .and(p.passport_id.as_ref())
        // .and(p.country_id.as_ref())
        .is_some()
}
