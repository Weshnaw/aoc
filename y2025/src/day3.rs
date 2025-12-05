fn calculate_joltage() -> usize {
    0
}


pub fn puzzle(_input: &str) -> usize {

    0
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_example_input() {
        let result = puzzle("987654321111111
811111111111119
234234234234278
818181911112111");
        assert_eq!(result, 3570);
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day3_input.txt"));
        assert_eq!(result, 0);
    }
}