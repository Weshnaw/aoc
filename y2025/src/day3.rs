pub fn puzzle(_input: &str) -> usize {
    todo!("puzzle");
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result, 0);
    }

    #[traced_test]
    #[test]
    fn test_example_input() {
        let result = puzzle("987654321111111
811111111111119
234234234234278
818181911112111");
        assert_eq!(result, 3570);
    }

    #[traced_test]
    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day3_input.txt"));
        assert_eq!(result, 0);
    }
}