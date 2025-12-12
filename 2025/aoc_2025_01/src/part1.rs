use super::Result;

pub fn process(input: impl AsRef<str>) -> Result<String> {
    println!("{}", input.as_ref());
    Ok("".to_string())
}
