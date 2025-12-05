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
        let result = puzzle("");
        assert_eq!(result, 0);
    }

    #[traced_test]
    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day0_input.txt"));
        assert_eq!(result, 0);
    }
}