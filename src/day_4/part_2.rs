use crate::day_4::passport::{
    BirthYear, CountryId, ExpirationYear, EyeColor, HairColor, Height, IssueYear, Passport,
    PassportId,
};
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let count = super::PASSPORTS
        .iter()
        .filter(validator)
        .count();
    println!("day_04::part_2:\tValid passport count: {}", count);
    Ok(())
}

fn validator(p: &&Passport) -> bool {
    Some("")
        .and(p.birth_year.as_ref())
        .filter(birth_year_filter)
        .and(p.issue_year.as_ref())
        .filter(issue_year_filter)
        .and(p.expiration_year.as_ref())
        .filter(expiration_year_filter)
        .and(p.height.as_ref())
        .filter(height_filter)
        .and(p.hair_color.as_ref())
        .filter(hair_color_filter)
        .and(p.eye_color.as_ref())
        .filter(eye_color_filter)
        .and(p.passport_id.as_ref())
        .filter(passport_id_filter)
        // .and(p.country_id.as_ref())
        // .filter(country_id_filter)
        .is_some()
}

fn birth_year_filter(birth_year: &&BirthYear) -> bool {
    let birth_year_value = birth_year.parse::<usize>().ok();
    birth_year.len() == 4 && birth_year_value >= Some(1920) && birth_year_value <= Some(2002)
}

fn issue_year_filter(issue_year: &&IssueYear) -> bool {
    let issue_year_value = issue_year.parse::<usize>().ok();
    issue_year.len() == 4 && issue_year_value >= Some(2010) && issue_year_value <= Some(2020)
}

fn expiration_year_filter(expiration_year: &&ExpirationYear) -> bool {
    let expiration_year_value = expiration_year.parse::<usize>().ok();
    expiration_year.len() == 4 && expiration_year_value >= Some(2020) && expiration_year_value <= Some(2030)
}

fn height_filter(height: &&Height) -> bool {
    let value = height.value.parse::<usize>().ok();
    if height.units.eq("cm") {
        value >= Some(150) && value <= Some(193)
    } else if height.units.eq("in") {
        value >= Some(59) && value <= Some(76)
    } else {
        false
    }
}

fn hair_color_filter(hair_color: &&HairColor) -> bool {
    let mut hair_color_chars = hair_color.chars();
    hair_color.len() == 7
        && hair_color_chars.next() == Some('#')
        && hair_color_chars.all(|c| char::is_ascii_hexdigit(&c))
}

fn eye_color_filter(eye_color: &&EyeColor) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color.as_str())
}

fn passport_id_filter(passport_id: &&PassportId) -> bool {
    passport_id.len() == 9 && passport_id.chars().all(char::is_numeric)
}

#[allow(dead_code)]
fn country_id_filter(country_id: &&CountryId) -> bool {
    country_id.chars().all(char::is_numeric)
}
