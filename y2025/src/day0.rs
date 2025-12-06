pub fn puzzle(_input: &str) -> (u64, u64) {
    todo!("puzzle");
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_example_input() {
        let result = puzzle("");
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day0_input.txt"));
        assert_eq!(result, (0, 0));
    }
}