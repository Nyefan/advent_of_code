pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    if s.is_empty() {
        return result;
    }
    let starting_palindromic_substrings = get_palindromic_substrings_starting_at_the_beginning(&s);
    for sps in starting_palindromic_substrings {
        if sps.len() == s.len() {
            result.push(vec![s.clone()]);
            continue;
        }
        let finishing_palindromic_substrings = partition(s[sps.len()..].to_string());
        for fps in finishing_palindromic_substrings {
            let mut next = vec![sps.clone()];
            next.extend(fps);
            result.push(next);
        }
    }
    result
}

fn get_palindromic_substrings_starting_at_the_beginning(s: &str) -> Vec<String> {
    let mut palindromic_substrings = Vec::new();
    for i in 0..s.len() {
        let substring = &s[0..=i];
        if is_palindrome(substring) {
            palindromic_substrings.push(substring.to_string());
        }
    }
    palindromic_substrings
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("aab", vec![vec!["a".to_string(), "a".to_string(), "b".to_string()], vec!["aa".to_string(), "b".to_string()]])]
    #[case("a", vec![vec!["a".to_string()]])]
    #[case("aabbaa", vec![
        vec!["a".to_string(), "a".to_string(), "b".to_string(), "b".to_string(), "a".to_string(), "a".to_string()],
        vec!["a".to_string(), "a".to_string(), "b".to_string(), "b".to_string(), "aa".to_string()],
        vec!["a".to_string(), "a".to_string(), "bb".to_string(), "a".to_string(), "a".to_string()],
        vec!["a".to_string(), "a".to_string(), "bb".to_string(), "aa".to_string()],
        vec!["a".to_string(), "abba".to_string(), "a".to_string()],
        vec!["aa".to_string(), "b".to_string(), "b".to_string(), "a".to_string(), "a".to_string()],
        vec!["aa".to_string(), "b".to_string(), "b".to_string(), "aa".to_string()],
        vec!["aa".to_string(), "bb".to_string(), "a".to_string(), "a".to_string()],
        vec!["aa".to_string(), "bb".to_string(), "aa".to_string()],
        vec!["aabbaa".to_string()],
    ])]
    fn test_partition(#[case] s: String, #[case] expected: Vec<Vec<String>>) {
        let result = partition(s);
        assert_eq!(result, expected);
    }
}
