pub fn contains_chinese(s: &str) -> bool {
    s.chars().any(|c| '\u{4E00}' <= c && c <= '\u{9FA5}')
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_chinese() {
        assert_eq!(contains_chinese("hello"), false);
        assert_eq!(contains_chinese("你好,443.,.24,.。，/234./2，5@#%@#》%《？@#……《"), true);
        assert_eq!(contains_chinese("hello你好"), true);
    }
}