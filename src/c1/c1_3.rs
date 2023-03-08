pub fn urlify(input: &mut str, length: usize) -> &mut str {
    let spaces = input.bytes().take(length).filter(|&b| b == b' ').count();
    let new_length = spaces * 2 + length;
    unsafe {
        let bytes = input.as_bytes_mut();
        let mut spaces_left = spaces;
        for i in (0..length).rev() {
            let offset = spaces_left * 2;
            if bytes[i] == b' ' {
                bytes[offset + i - 2] = b'%';
                bytes[offset + i - 1] = b'2';
                bytes[offset + i] = b'0';
                spaces_left -= 1;
            } else {
                bytes[offset + i] = bytes[i];
            }
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_urlify() {
        assert_eq!(urlify(&mut String::from("hej"), 3), "hej");
        assert_eq!(urlify(&mut String::from("abc efgXX"), 7), "abc%20efg");
        assert_eq!(urlify(&mut String::from("  xxXX"), 2), "%20%20");
        assert_eq!(urlify(&mut String::from(" a b xxXXxx"), 5), "%20a%20b%20");
    }
}
