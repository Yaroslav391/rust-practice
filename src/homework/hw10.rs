pub fn is_palindromic_number(n: u64) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}
mod tests {
    use super::*;

    #[test]
    fn test_palindromic_numbers() {
        assert!(is_palindromic_number(0));
        assert!(is_palindromic_number(1));
        assert!(is_palindromic_number(121));
        assert!(is_palindromic_number(12321));
        assert!(is_palindromic_number(4444444444444444));
    }
    fn test_non_palindromic_numbers() {
        assert!(!is_palindromic_number(10));
        assert!(!is_palindromic_number(123));
        assert!(!is_palindromic_number(123456));
    }
}
