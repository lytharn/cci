#[allow(dead_code)]
pub fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    (s1.to_string() + s1).contains(s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_rotation() {
        assert!(is_rotation("", ""));
        assert!(is_rotation("cat", "cat"));
        assert!(is_rotation("atc", "cat"));
        assert!(is_rotation("waterbottle", "erbottlewat"));
    }

    #[test]
    fn should_not_be_rotation() {
        assert!(!is_rotation("waterbottle", "water"));
        assert!(!is_rotation("water", "waterbottle"));
        assert!(!is_rotation("waterbottle", "waterbuttle"));
    }
}
