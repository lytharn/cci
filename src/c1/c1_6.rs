#[allow(dead_code)]
pub fn compress_str(to_compress: &str) -> String {
    let mut compressed = String::new();
    let mut chars = to_compress.chars();
    if let Some(first) = chars.next() {
        let mut last = first;
        let mut same_as_last: u32 = 1;
        for c in chars {
            if c == last {
                same_as_last += 1;
            } else {
                compressed.push(last);
                compressed.push_str(&same_as_last.to_string());
                last = c;
                same_as_last = 1;
            }
        }
        compressed.push(last);
        compressed.push_str(&same_as_last.to_string());
    }
    if compressed.len() <= to_compress.len() {
        compressed
    } else {
        to_compress.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compress_str_if_equal_or_smaller_in_size() {
        assert_eq!(compress_str(""), "");
        assert_eq!(compress_str("aa"), "a2");
        assert_eq!(compress_str("aaabbc"), "a3b2c1");
        assert_eq!(compress_str("aaaaa"), "a5");
    }

    #[test]
    fn should_return_same_string_if_compressed_is_bigger_in_size() {
        assert_eq!(compress_str("a"), "a");
        assert_eq!(compress_str("aaabbcd"), "aaabbcd");
    }
}
