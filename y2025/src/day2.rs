use itertools::Itertools;
use rayon::prelude::*;

fn halves_match(id: &usize) -> bool {
    if id == &0 {
        return false;
    }
    
    let digit_count = id.ilog10() + 1;

    if digit_count % 2 != 0 {
        return false;
    }

    let half_point = 10usize.pow(digit_count / 2);

    // Compare the halves
    (id / half_point) == (id % half_point)
}

pub fn puzzle(input: &str) -> usize {
    let result = input.par_split(',').map(move |ids| {
        let (start, end) = ids.splitn(2, '-').map(|n| n.parse::<usize>().unwrap_or_default()).collect_tuple().unwrap_or_default();

        (start..=end).into_par_iter().filter(halves_match).sum::<usize>()
    }).sum::<usize>();

    result
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tracing_test::traced_test;

    use super::*;
    
    #[traced_test]
    #[rstest]
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
    fn test_valid_halves_match(#[case] input: usize, #[case] expected: bool) {
        assert_eq!(halves_match(&input), expected);
    }

    #[traced_test]
    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result, 0);
    }

    #[traced_test]
    #[test]
    fn test_example_input() {
        let result = puzzle("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
        assert_eq!(result, 1227775554);
    }

    #[traced_test]
    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day2_input.txt"));
        assert_eq!(result, 24157613387);
    }
}