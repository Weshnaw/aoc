use std::cmp::Ordering;

use rayon::prelude::*;
use tracing::info;
use winnow::Parser;

pub fn puzzle(input: &str) -> (u64, u64) {
    let input = Ingredients::from_str(input).merge_ranges();

    (input.count_fresh(), input.total_fresh())
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
pub struct Ingredients {
    fresh: Vec<FreshRange>,
    inventory: Vec<u64>,
}

impl Ingredients {
    fn from_str(input: &str) -> Self {
        parsing::parse_ingredients.parse(input).unwrap_or_default()
    }

    pub fn count_fresh(&self) -> u64 {
        self.inventory
            .par_iter()
            .filter(|ingredient| {
                self.fresh
                    .par_iter()
                    .any(|fresh| **ingredient >= fresh.start && **ingredient <= fresh.end)
            })
            .count() as u64
    }

    fn merge_ranges(mut self) -> MergedIngredients {
        self.fresh.sort_by_key(|f| f.start);

        let mut merged: Vec<FreshRange> = Vec::with_capacity(self.fresh.len());

        for current_fresh in &self.fresh {
            match merged.last_mut() {
                Some(last) if current_fresh.start <= last.end => {
                    last.end = last.end.max(current_fresh.end);
                }
                _ => merged.push(*current_fresh),
            }
            info!(?merged);
        }

        MergedIngredients {
            fresh: merged,
            inventory: self.inventory,
        }
    }
}

#[derive(Debug, PartialEq)]
struct MergedIngredients {
    fresh: Vec<FreshRange>,
    inventory: Vec<u64>,
}

impl MergedIngredients {
    fn count_fresh(&self) -> u64 {
        self.inventory
            .par_iter()
            .filter(|item| {
                self.fresh
                    .binary_search_by(|r| {
                        if **item < r.start {
                            Ordering::Greater
                        } else if **item > r.end {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    })
                    .is_ok()
            })
            .count() as u64
    }

    fn total_fresh(&self) -> u64 {
        self.fresh.par_iter().map(|r| r.end - r.start + 1).sum()
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
                inventory: parse_inventory_list,
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
    use rstest::rstest;
    use test_log::test;

    use super::*;

    #[test]
    #[rstest]
    #[case(Ingredients {fresh: vec![], ..Default::default()}, 0)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 10, end: 20}], ..Default::default()}, 11)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 10, end: 20}, FreshRange {start: 10, end: 20}], ..Default::default()}, 11)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 10, end: 20}, FreshRange {start:  5, end: 25}], ..Default::default()}, 21)]
    #[case(Ingredients {fresh: vec![FreshRange {start:  5, end: 25}, FreshRange {start: 10, end: 20}], ..Default::default()}, 21)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 15, end: 25}, FreshRange {start: 10, end: 20}], ..Default::default()}, 16)]
    #[case(Ingredients {fresh: vec![FreshRange {start:  5, end: 15}, FreshRange {start: 10, end: 20}], ..Default::default()}, 16)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 10, end: 20}, FreshRange {start: 15, end: 25}], ..Default::default()}, 16)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 10, end: 20}, FreshRange {start:  5, end: 15}], ..Default::default()}, 16)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 20, end: 25}, FreshRange {start: 10, end: 20}], ..Default::default()}, 16)]
    #[case(Ingredients {fresh: vec![FreshRange {start:  5, end: 10}, FreshRange {start: 10, end: 20}], ..Default::default()}, 16)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 10, end: 20}, FreshRange {start: 20, end: 25}], ..Default::default()}, 16)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 10, end: 20}, FreshRange {start:  5, end: 10}], ..Default::default()}, 16)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 10, end: 20}, FreshRange {start: 30, end: 40}], ..Default::default()}, 22)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 30, end: 40}, FreshRange {start: 10, end: 20}], ..Default::default()}, 22)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 10, end: 20}, FreshRange {start: 30, end: 40}, FreshRange {start: 15, end: 35}], ..Default::default()}, 31)]
    #[case(Ingredients {fresh: vec![FreshRange {start: 3, end: 5}, FreshRange {start: 10, end: 14}, FreshRange {start: 16, end: 20}, FreshRange {start: 12, end: 18}], ..Default::default()}, 14)]
    fn test_total_fresh(#[case] inventory: Ingredients, #[case] expected: u64) {
        let result = inventory.merge_ranges().total_fresh();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result, (0, 0));
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
        assert_eq!(result, (3, 14));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day5_input.txt"));
        assert_eq!(result, (720, 357608232770687));
    }
}
