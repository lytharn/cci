use std::collections::HashSet;

pub fn all_chars_unique(input: &str) -> bool {
    let mut chars = HashSet::new();
    for c in input.chars() {
        if chars.contains(&c) {
            return false;
        }
        chars.insert(c);
    }
    true
}

pub fn all_chars_unique2(input: &str) -> bool {
    let mut sorted: Vec<char> = input.chars().collect();
    sorted.sort_by(|a, b| b.cmp(a));
    let mut prev_c: Option<char> = None;
    for c in sorted {
        if prev_c == Some(c) {
            return false;
        }
        prev_c = Some(c);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_should_return_true() {
        assert!(all_chars_unique("abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn unique_should_return_true2() {
        assert!(all_chars_unique2("abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn non_unique_should_return_false() {
        assert!(!all_chars_unique("aa"));
        assert!(!all_chars_unique("abcdefghijklmnopqrstuvwxyza"));
    }

    #[test]
    fn non_unique_should_return_false2() {
        assert!(!all_chars_unique2("aa"));
        assert!(!all_chars_unique2("abcdefghijklmnopqrstuvwxyza"));
    }
}
