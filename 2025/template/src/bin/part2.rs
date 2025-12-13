use {{crate_name}}::part2::process;
use {{crate_name}}::Result;

#[tracing::instrument]
pub fn main() -> Result<'static, ()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../data/part2.txt");
    let result = process(file)?;
    println!("{:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use {{crate_name}}::part2::process;

    #[test]
    fn test() {
        let input = "";
        assert_eq!("", process(input).unwrap());
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../data/part2.txt");
        assert_eq!("", process(input).unwrap());
    }
}
