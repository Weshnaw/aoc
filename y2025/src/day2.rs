use itertools::Itertools;
use rayon::prelude::*;

fn two_match(id: &usize) -> bool {
    if id == &0 {
        return false;
    }

    let digit_count = id.ilog10() + 1;

    if !digit_count.is_multiple_of(2) {
        return false;
    }

    let half_base = 10usize.pow(digit_count / 2);

    let pattern = id % half_base;

    id / half_base == pattern
}

fn check_all_blocks_match(mut id: usize, pattern: usize, block_base: usize) -> bool {
    while id > 0 {
        if id % block_base != pattern {
            return false;
        }
        id /= block_base;
    }
    true
}

fn multiple_match(id: &usize) -> bool {
    if id == &0 {
        return false;
    }

    let digit_count = id.ilog10() as usize + 1;

    for block_len in 1..=digit_count / 2 {
        // block must evenly divide the number of digits
        if !digit_count.is_multiple_of(block_len) {
            continue;
        }

        let block_base = 10usize.pow(block_len as u32);
        // get the pattern for the block size
        let pattern = id % block_base;

        if check_all_blocks_match(*id, pattern, block_base) {
            return true;
        }
    }

    false
}

pub fn puzzle(input: &str) -> (usize, usize) {
    input
        .par_split(',')
        .map(move |ids| {
            let (start, end) = ids
                .splitn(2, '-')
                .map(|n| n.parse::<usize>().unwrap_or_default())
                .collect_tuple()
                .unwrap_or_default();

            (
                (start..=end)
                    .into_par_iter()
                    .filter(two_match)
                    .sum::<usize>(),
                (start..=end)
                    .into_par_iter()
                    .filter(multiple_match)
                    .sum::<usize>(),
            )
        })
        .reduce(
            || (0, 0),
            |result, acc| (result.0 + acc.0, result.1 + acc.1),
        )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use test_log::test;

    use super::*;

    #[test]
    #[rstest]
    #[case(0, false)]
    #[case(11, true)]
    #[case(22, true)]
    #[case(99, true)]
    #[case(1010, true)]
    #[case(1188511885, true)]
    #[case(222222, true)]
    #[case(446446, true)]
    #[case(38593859, true)]
    #[case(13, false)]
    #[case(95, false)]
    #[case(1188511880, false)]
    #[case(222220, false)]
    #[case(1698522, false)]
    #[case(446443, false)]
    #[case(38593856, false)]
    fn test_two_match(#[case] input: usize, #[case] expected: bool) {
        assert_eq!(two_match(&input), expected);
    }

    #[test]
    #[rstest]
    #[case(0, false)]
    #[case(11, true)]
    #[case(22, true)]
    #[case(99, true)]
    #[case(1010, true)]
    #[case(1188511885, true)]
    #[case(222222, true)]
    #[case(446446, true)]
    #[case(38593859, true)]
    #[case(13, false)]
    #[case(95, false)]
    #[case(1188511880, false)]
    #[case(222220, false)]
    #[case(1698522, false)]
    #[case(446443, false)]
    #[case(38593856, false)]
    #[case(123123123, true)]
    #[case(1212121212, true)]
    #[case(1111111, true)]
    #[case(12312312, false)]
    fn test_multi_match(#[case] input: usize, #[case] expected: bool) {
        assert_eq!(multiple_match(&input), expected);
    }

    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_example_input() {
        let result = puzzle(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(result, (1227775554, 4174379265));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day2_input.txt"));
        assert_eq!(result, (24157613387, 33832678380));
    }
}
