pub fn count_and_say(n: i32) -> String {
    let mut result = "1".to_string();
    for _ in 1..n {
        result = encode(result);
    }
    result
}

fn encode(s: String) -> String {
    let mut chars = s.chars();
    let mut current_val = chars.next().unwrap();
    let mut num_current = 1;
    let mut result = String::new();
    for c in chars {
        if c == current_val {
            num_current += 1;
        } else {
            result.push_str(&num_current.to_string());
            result.push(current_val);
            current_val = c;
            num_current = 1;
        }
    }
    result.push_str(&num_current.to_string());
    result.push(current_val);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, "1".to_string())]
    #[case(4, "1211".to_string())]
    #[case(6, "312211".to_string())]
    fn test_count_and_say(#[case] input: i32, #[case] output: String) {
        assert_eq!(count_and_say(input), output);
    }
}
