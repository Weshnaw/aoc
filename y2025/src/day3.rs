use rayon::prelude::*;
use tracing::{debug, trace};

fn calculate_joltage<const N: usize>(input: &str) -> u64 {
    let input = input.trim();
    if input.is_empty() {
        return 0;
    }

    let digits = input
        .as_bytes()
        .iter()
        .rev()
        .enumerate()
        .fold([0u64; N], |mut current_digits, (idx, current_character)| {
            let num = (current_character & 0x0F) as u64;

            if idx < N {
                current_digits[idx] = num;
            } else {
                debug!("Cascading {num} with {current_digits:?}");
                cascading_update(&mut current_digits, num);
            }

            current_digits
        });

    let num: u64 = digits
        .iter()
        .enumerate()
        .map(|(idx, digit)| 10u64.pow(idx as u32) * digit)
        .sum();

    debug!("'{input}' -> '{num}");
    num
}

fn cascading_update<const N: usize>(digits: &mut [u64; N], mut num: u64) {
    trace!("Cascade: num={num} len={}, digits={digits:?}", digits.len());
    for i in (0..digits.len()).rev() {
        trace!(i, num);
        if digits[i] <= num {
            trace!("swapped at idx={i} num={num} digit={}", digits[i]);
            (digits[i], num) = (num, digits[i]);
        } else {
            break;
        }
    }
    trace!("Cascade result: num={num} len={}, digits={digits:?}", digits.len());
}

fn calculate_both(input: &str) -> (u64, u64) {
    (calculate_joltage::<2>(input), calculate_joltage::<12>(input))
}

pub fn puzzle(input: &str) -> (u64, u64) {
    input
        .trim()
        .par_lines()
        .map(calculate_both)
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
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
    fn test_calculate_joltage(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(calculate_joltage::<2>(input), expected);
    }

    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_example_input() {
        let result = puzzle(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(result, (357, 3121910778619));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day3_input.txt"));
        assert_eq!(result, (17155, 169685670469164));
    }
}
