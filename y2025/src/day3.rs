use rayon::prelude::*;
use tracing::{debug, info};

fn calculate_joltage<const N: usize>(input: &str) -> u64 {
    let bytes = input.trim().as_bytes();
    if bytes.len() < N {
        return byte_digts_to_number(bytes);
    }

    let mut digits = [0u8; N];
    let mut previous_digit_idx: usize = 0;
    info!(?bytes);
    for idx in 0..N {
        debug!(?idx, ?digits, ?previous_digit_idx);
        let slice_end = bytes.len() - (N - idx - 1);
        let slice = &bytes[previous_digit_idx..slice_end];
        let (digit_idx, max) = slice.iter().enumerate().rev().max_by_key(|n| n.1).unwrap();
        debug!(?slice, ?digit_idx, ?max);
        if digit_idx == (slice.len() - 1) {
            info!("Skipping rest");
            digits[idx..].copy_from_slice(&bytes[previous_digit_idx + digit_idx..]);
            debug!(?digits);
            break;
        }
        previous_digit_idx += digit_idx + 1;
        digits[idx] = *max;
    }

    let num = byte_digts_to_number(&digits);

    info!("'{input}' -> '{num}");
    num
}

fn byte_digts_to_number(digits: &[u8]) -> u64 {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, digit)| {
            let num = (digit & 0x0F) as u64;
            10u64.pow(idx as u32) * num
        })
        .sum()
}

fn calculate_both(input: &str) -> (u64, u64) {
    (
        calculate_joltage::<2>(input),
        calculate_joltage::<12>(input),
    )
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
    fn test_calculate_joltage_2(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(calculate_joltage::<2>(input), expected);
    }
    #[test]
    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    #[case("999999999999998", 999999999999)]
    #[case("11", 11)]
    #[case("1", 1)]
    fn test_calculate_joltage_12(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(calculate_joltage::<12>(input), expected);
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
