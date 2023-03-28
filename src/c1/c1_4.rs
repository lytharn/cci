use std::collections::HashMap;

#[allow(dead_code)]
pub fn is_palindrome_permutable(text: &str) -> bool {
    let lower_text = text.to_lowercase();
    let mut frequency: HashMap<char, usize> = HashMap::new();
    lower_text
        .chars()
        .filter(|&c| c != ' ')
        .for_each(|c| *frequency.entry(c).or_insert(0) += 1);
    frequency.values().filter(|&x| x % 2 == 1).count() < 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_if_a_permutation_of_a_palindrome() {
        assert!(is_palindrome_permutable("aab"));
        assert!(is_palindrome_permutable("ababcc"));
        assert!(is_palindrome_permutable("kayak"));
    }

    #[test]
    fn should_return_false_if_not_a_permutation_of_a_palindrome() {
        assert!(!is_palindrome_permutable("ab"));
        assert!(!is_palindrome_permutable("abababcc"));
        assert!(!is_palindrome_permutable("kayak2"));
    }

    #[test]
    fn should_ignore_whitespace() {
        assert!(is_palindrome_permutable("a "));
        assert!(is_palindrome_permutable(" a  "));
        assert!(is_palindrome_permutable("b a b "));
        assert!(!is_palindrome_permutable("a b "));
    }
}
