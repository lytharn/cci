#[allow(dead_code)]
pub fn is_one_character_edit_away(input: &str, result: &str) -> bool {
    if input.len().abs_diff(result.len()) > 1 {
        return false;
    }
    if input.len() < result.len() {
        return is_one_add_away(input, result);
    }
    if input.len() > result.len() {
        return is_one_add_away(result, input);
    }
    is_one_change_away(input, result)
}

fn is_one_add_away(input: &str, result: &str) -> bool {
    let mut input_chars = input.chars();
    let mut result_chars = result.chars();
    while let Some(c) = input_chars.next() {
        if Some(c) != result_chars.next() {
            result_chars.next();
            return input_chars.eq(result_chars);
        }
    }
    true
}

fn is_one_change_away(input: &str, result: &str) -> bool {
    input
        .chars()
        .zip(result.chars())
        .filter(|(ci, cr)| ci != cr)
        .count()
        <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_if_zero_edit_away() {
        assert!(is_one_character_edit_away("", ""));
        assert!(is_one_character_edit_away("a", "a"));
        assert!(is_one_character_edit_away("Mercury", "Mercury"));
    }

    #[test]
    fn should_return_true_if_one_remove_away() {
        assert!(is_one_character_edit_away("a", ""));
        assert!(is_one_character_edit_away("abcd", "bcd"));
        assert!(is_one_character_edit_away("efgh", "egh"));
        assert!(is_one_character_edit_away("ijklm", "jklm"));
    }

    #[test]
    fn should_return_false_if_more_than_one_remove_away() {
        assert!(!is_one_character_edit_away("aa", ""));
        assert!(!is_one_character_edit_away("abcd", "cd"));
        assert!(!is_one_character_edit_away("efgh", "h"));
    }

    #[test]
    fn should_return_true_if_one_add_away() {
        assert!(is_one_character_edit_away("", "x"));
        assert!(is_one_character_edit_away("abcd", "abcde"));
        assert!(is_one_character_edit_away("ghiklm", "ghijklm"));
        assert!(is_one_character_edit_away("xyz", "wxyz"));
    }

    #[test]
    fn should_return_false_if_more_than_one_add_away() {
        assert!(!is_one_character_edit_away("", "xx"));
        assert!(!is_one_character_edit_away("abcd", "abcdef"));
        assert!(!is_one_character_edit_away("ghklm", "ghijklm"));
        assert!(!is_one_character_edit_away("yz", "wxyz"));
    }

    #[test]
    fn should_return_true_if_one_change_away() {
        assert!(is_one_character_edit_away("a", "b"));
        assert!(is_one_character_edit_away("bbcdef", "abcdef"));
        assert!(is_one_character_edit_away("ghjjklm", "ghijklm"));
        assert!(is_one_character_edit_away("wxya", "wxyz"));
    }

    #[test]
    fn should_return_false_if_more_than_one_change_away() {
        assert!(!is_one_character_edit_away("ab", "cd"));
        assert!(!is_one_character_edit_away("bbbdef", "abcdef"));
        assert!(!is_one_character_edit_away("ghjjjlm", "ghijklm"));
        assert!(!is_one_character_edit_away("axya", "wxyz"));
    }
}
