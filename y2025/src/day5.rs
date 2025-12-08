use std::ops::RangeInclusive;

use itertools::Itertools;
use rangemap::RangeInclusiveSet;
use rayon::prelude::*;
use winnow::Parser;

pub fn puzzle(input: &str) -> (u64, u64) {
    let input = Ingredients::from_str(input);

    (input.count_fresh(), input.total_fresh())
}
// use RangeInclusive instead of FreshRange
// use rangemap::RangeInclusiveSet for part 2
#[derive(Debug, PartialEq, Default)]
pub struct Ingredients {
    fresh: RangeInclusiveSet<u64>,
    inventory: Vec<u64>,
}

impl Ingredients {
    fn from_str(input: &str) -> Self {
        RangeInclusiveSet::from_iter(vec![RangeInclusive::new(0, 0)]);
        parsing::parse_ingredients.parse(input).unwrap_or_default()
    }
    fn count_fresh(&self) -> u64 {
        self.inventory
            .par_iter()
            .filter(|item| self.fresh.contains(item))
            .count() as u64
    }

    fn total_fresh(&self) -> u64 {
        self.fresh
            .iter()
            .map(|f| f.try_len().unwrap())
            .sum::<usize>() as u64
    }
}

mod parsing {
    use std::ops::RangeInclusive;

    use rangemap::RangeInclusiveSet;
    use winnow::{
        ascii::{digit1, line_ending},
        combinator::{separated, seq},
        prelude::*,
    };

    use crate::day5::Ingredients;

    fn parse_fresh_range(input: &mut &str) -> winnow::Result<RangeInclusive<u64>> {
        seq!(
            digit1.try_map(str::parse),
                _: '-',
                digit1.try_map(str::parse)
        )
        .map(|(start, end)| RangeInclusive::new(start, end))
        .parse_next(input)
    }

    fn parse_fresh_range_list(input: &mut &str) -> winnow::Result<Vec<RangeInclusive<u64>>> {
        separated(0.., parse_fresh_range, line_ending).parse_next(input)
    }

    fn parse_inventory_list(input: &mut &str) -> winnow::Result<Vec<u64>> {
        separated(0.., digit1.try_map(str::parse::<u64>), line_ending)
            // .map(|(a, _)| a)
            .parse_next(input)
    }

    pub fn parse_ingredients(input: &mut &str) -> winnow::Result<Ingredients> {
        seq!(
            Ingredients {
                fresh: parse_fresh_range_list.map(RangeInclusiveSet::from_iter),
                _: line_ending,
                _: line_ending,
                inventory: parse_inventory_list,
            }
        )
        .parse_next(input)
    }

    #[cfg(test)]
    mod tests {
        use rstest::rstest;
        use test_log::test;

        use crate::day5::Ingredients;

        use super::*;

        #[test]
        #[rstest]
        #[case("123-123", RangeInclusive::new(123, 123))]
        fn test_parse_fresh_range(#[case] input: &str, #[case] expected: RangeInclusive<u64>) {
            let (left_over, result) = parse_fresh_range.parse_peek(input).unwrap();

            assert_eq!(left_over, "");
            assert_eq!(result, expected);
        }

        #[test]
        #[rstest]
        #[case("123-123
123-123", vec![RangeInclusive::new(123, 123), RangeInclusive::new(123, 123)])]
        fn test_parse_fresh_range_list(
            #[case] input: &str,
            #[case] expected: Vec<RangeInclusive<u64>>,
        ) {
            let (left_over, result) = parse_fresh_range_list.parse_peek(input).unwrap();

            assert_eq!(left_over, "");
            assert_eq!(result, expected);
        }

        #[test]
        #[rstest]
        #[case("1
2
3
4
5", vec![1u64, 2, 3, 4, 5])]
        fn test_parse_inventory_list(#[case] input: &str, #[case] expected: Vec<u64>) {
            let (left_over, result) = parse_inventory_list.parse_peek(input).unwrap();

            assert_eq!(left_over, "");
            assert_eq!(result, expected);
        }

        #[test]
        #[rstest]
        #[case("123-123
123-123

1
2
3
4
5", Ingredients { fresh: RangeInclusiveSet::from_iter(vec![RangeInclusive::new(123, 123)]), inventory: vec![1u64, 2, 3, 4, 5]})]
        fn test_parse_ingredients(#[case] input: &str, #[case] expected: Ingredients) {
            let (left_over, result) = parse_ingredients.parse_peek(input).unwrap();

            assert_eq!(left_over, "");
            assert_eq!(result, expected);
        }
    }
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
        let result = puzzle(
            "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert_eq!(result, (3, 14));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day5_input.txt"));
        assert_eq!(result, (720, 357608232770687));
    }
}
