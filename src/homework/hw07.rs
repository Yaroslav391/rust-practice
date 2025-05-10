pub fn invert_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else if c.is_lowercase() {
                c.to_uppercase().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}
mod tests {
    use super::*;
    fn test_invert_case() {
        assert_eq!(invert_case("Hello World!"), "hELLO wORLD!");
        assert_eq!(invert_case("RUST"), "rust");
        assert_eq!(invert_case("rust"), "RUST");
        assert_eq!(invert_case("123ABCabc!"), "123abcABC!");
    }
}
