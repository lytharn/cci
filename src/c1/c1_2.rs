use std::collections::HashMap;

pub fn is_permutation_equal(a: &str, b: &str) -> bool {
    count_chars(a) == count_chars(b)
}

fn count_chars(string: &str) -> HashMap<char, i32> {
    let mut count_map = HashMap::new();
    for c in string.chars() {
        count_map
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    count_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_permutation_equal() {
        assert!(is_permutation_equal("", ""));
        assert!(is_permutation_equal("aaa", "aaa"));
        assert!(is_permutation_equal("aabbcc", "cbabac"));
    }

    #[test]
    fn should_not_be_permutation_equal() {
        assert!(!is_permutation_equal("a", ""));
        assert!(!is_permutation_equal("", "a"));
        assert!(!is_permutation_equal("aaa", "aaaa"));
        assert!(!is_permutation_equal("aabbccc", "cbabac"));
    }
}
