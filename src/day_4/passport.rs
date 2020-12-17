use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

pub type BirthYear = String;
pub type IssueYear = String;
pub type ExpirationYear = String;
#[derive(Debug)]
pub struct Height {
    pub units: String,
    pub value: String,
}
pub type HairColor = String;
pub type EyeColor = String;
pub type PassportId = String;
pub type CountryId = String;

#[derive(Debug)]
pub struct Passport {
    pub birth_year: Option<BirthYear>,
    pub issue_year: Option<IssueYear>,
    pub expiration_year: Option<ExpirationYear>,
    pub height: Option<Height>,
    pub hair_color: Option<HairColor>,
    pub eye_color: Option<EyeColor>,
    pub passport_id: Option<PassportId>,
    pub country_id: Option<CountryId>,
}

impl FromStr for Passport {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: HashMap<String, String> = s
            .split_whitespace()
            .map(|entry| {
                let mut iter = entry.split(':').map(String::from);
                (iter.next().unwrap(), iter.next().unwrap())
            })
            .collect();
        Ok(Passport {
            birth_year: values.get_parsed("byr"),
            issue_year: values.get_parsed("iyr"),
            expiration_year: values.get_parsed("eyr"),
            height: values.get_parsed("hgt"),
            hair_color: values.get_parsed("hcl"),
            eye_color: values.get_parsed("ecl"),
            passport_id: values.get_parsed("pid"),
            country_id: values.get_parsed("cid"),
        })
    }
}

impl FromStr for Height {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, units) = s.split_at(s.len() - 2);
        let (units, value) = (units.parse::<String>()?, value.parse::<String>()?);
        Ok(Height { units, value })
    }
}

trait GetParsed {
    fn get_parsed<T: FromStr>(self: &Self, identifier: &str) -> Option<T>;
}

impl GetParsed for HashMap<String, String> {
    fn get_parsed<T: FromStr>(self: &Self, identifier: &str) -> Option<T> {
        self.get(identifier).map(|s| s.parse::<T>().ok()).flatten()
    }
}
