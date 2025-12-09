pub fn puzzle(_input: &str) -> (u64, u64) {
    todo!("puzzle");
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_example_input() {
        let result = puzzle("\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3");
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day9_input.txt"));
        assert_eq!(result, (0, 0));
    }
}
