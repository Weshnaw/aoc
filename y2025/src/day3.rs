use rayon::prelude::*;
use tracing::{debug, warn};

fn calculate_joltage(input: &str) -> u32 {
    let input = input.trim();
    if input.is_empty() {
        return 0;
    }

    let (left_digit, right_digit) = input.chars().rev().fold((0, 0), |(mut left_digit, mut right_digit), current_character| {
        let num = current_character.to_digit(10).unwrap_or_default();

        if right_digit == 0 {
            right_digit = num;
        } else if num >= left_digit {
            if left_digit > right_digit {
                right_digit = left_digit;
            }
            left_digit = num;
        }

        (left_digit, right_digit)
    });

    if left_digit == 0 || right_digit == 0 {
        warn!("Probably issue for '{input}'")
    }
    
    let num = left_digit * 10 + right_digit;

    debug!("'{input}' -> '{num}");
    num
}


pub fn puzzle(input: &str) -> u32 {
    input.trim().par_lines().map(calculate_joltage).sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use test_log::test;

    use super::*;

    #[test]
    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("999999999999998", 99)]
    #[case("11", 11)]
    #[case("1", 1)]
    fn test_calculate_joltage(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(calculate_joltage(input), expected);
    }
    

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
        assert_eq!(result, 357);
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day3_input.txt"));
        assert_eq!(result, 17155);
    }
}