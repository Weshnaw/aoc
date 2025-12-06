use rayon::prelude::*;
use winnow::Parser;

pub fn puzzle(input: &str) -> u64 {
    let input = Ingredients::from_str(input);

    input.count_fresh()
}

#[derive(Debug, PartialEq)]
struct FreshRange {
    start: u64,
    end: u64,
}

impl From<(u64, u64)> for FreshRange {
    fn from(value: (u64, u64)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}

#[derive(Debug, PartialEq, Default)]
struct Ingredients {
    fresh: Vec<FreshRange>,
    inventory: Vec<u64>,
}

impl Ingredients {
    fn from_str(input: &str) -> Self {
        parsing::parse_ingredients.parse(input).unwrap_or_default()
    }

    fn count_fresh(&self) -> u64 {
        self.inventory
            .par_iter()
            .filter(|ingredient| {
                self.fresh
                    .par_iter()
                    .any(|fresh| **ingredient >= fresh.start && **ingredient <= fresh.end)
            })
            .count() as u64
    }
}

mod parsing {
    use winnow::{
        ascii::{digit1, line_ending},
        combinator::{separated, seq},
        prelude::*,
    };

    use crate::day5::{FreshRange, Ingredients};

    fn parse_fresh_range(input: &mut &str) -> winnow::Result<FreshRange> {
        seq!(
            FreshRange {
                start: digit1.try_map(str::parse),
                _: '-',
                end: digit1.try_map(str::parse)
            }
        )
        .parse_next(input)
    }

    fn parse_fresh_range_list(input: &mut &str) -> winnow::Result<Vec<FreshRange>> {
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
                fresh: parse_fresh_range_list,
                _: line_ending,
                _: line_ending,
                inventory: parse_inventory_list
            }
        )
        .parse_next(input)
    }

    #[cfg(test)]
    mod tests {
        use rstest::rstest;
        use test_log::test;

        use crate::day5::{FreshRange, Ingredients};

        use super::*;

        #[test]
        #[rstest]
        #[case("123-123", FreshRange { start: 123, end: 123})]
        fn test_parse_fresh_range(#[case] input: &str, #[case] expected: FreshRange) {
            let (left_over, result) = parse_fresh_range.parse_peek(input).unwrap();

            assert_eq!(left_over, "");
            assert_eq!(result, expected);
        }

        #[test]
        #[rstest]
        #[case("123-123
123-123", vec![FreshRange { start: 123, end: 123}, FreshRange { start: 123, end: 123}])]
        fn test_parse_fresh_range_list(#[case] input: &str, #[case] expected: Vec<FreshRange>) {
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
5", Ingredients { fresh: vec![FreshRange { start: 123, end: 123}, FreshRange { start: 123, end: 123}], inventory: vec![1u64, 2, 3, 4, 5]})]
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
        assert_eq!(result, 0);
    }

    #[test]
    fn test_example_input() {
        let result = puzzle(
            "3-5
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
        assert_eq!(result, 3);
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day5_input.txt"));
        assert_eq!(result, 720);
    }
}
