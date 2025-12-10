pub fn puzzle(_input: &str) -> (u64, u64) {
    todo!("puzzle");
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    const EXAMPLE: &str = "\
";
    const INPUT: &str = include_str!("day0_input.txt");

    #[test]
    fn test_example_input() {
        let result = puzzle(EXAMPLE);
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_input() {
        let result = puzzle(INPUT);
        assert_eq!(result, (0, 0));
    }
}
