use glam::U64Vec2;
use ndarray::Array2;
use tracing::info;
use winnow::Parser;

use crate::day12::parsing::parse_input;

pub fn puzzle(input: &str) -> usize {
    let (_, input) = parse_input.parse_peek(input).unwrap();

    let (trivial, non_trivial): (Vec<_>, Vec<_>) = input
        .trees
        .iter()
        .filter(|tree| {
            // filter out the trivially impossible ones i.e. the total present area > tree area
            tree.area()
                >= tree
                    .present_counts
                    .iter()
                    .enumerate()
                    .map(|(idx, count)| count * input.presents[idx].area())
                    .sum::<u64>()
        })
        .partition(|tree| tree.trimmed_area() >= (9 * tree.present_counts.iter().sum::<u64>()));

    info!(trivial = trivial.len(), non_trivial = non_trivial.len());

    trivial.len()
        + non_trivial
            .iter()
            .filter(|tree| solve_non_trivial_tree(tree, &input.presents))
            .count()
}

fn solve_non_trivial_tree(_tree: &Tree, _presents: &[Shape]) -> bool {
    // TODO: solve non-trivial cases
    true
}

#[derive(Debug)]
struct Input {
    presents: Vec<Shape>,
    trees: Vec<Tree>,
}

#[derive(Debug)]
struct Shape {
    shape: Array2<bool>,
}

impl Shape {
    fn area(&self) -> u64 {
        self.shape.iter().filter(|f| **f).count() as u64
    }
}

#[derive(Debug)]
struct Tree {
    dimensions: U64Vec2,
    present_counts: Vec<u64>,
}

impl Tree {
    fn area(&self) -> u64 {
        self.dimensions.x * self.dimensions.y
    }
    fn trimmed_area(&self) -> u64 {
        let trimmed_x = self.dimensions.x - self.dimensions.x % 3;
        let trimmed_y = self.dimensions.y - self.dimensions.y % 3;
        trimmed_x * trimmed_y
    }
}

mod parsing {
    use super::*;
    use winnow::{
        ascii::{digit1, multispace1, space1},
        combinator::{alt, preceded, repeat, separated, seq},
    };

    fn parse_taken_spaces(input: &mut &str) -> winnow::Result<Vec<bool>> {
        repeat(3, alt(("#", ".")).map(|char| char == "#")).parse_next(input)
    }

    fn parse_shape(input: &mut &str) -> winnow::Result<Shape> {
        separated(3, parse_taken_spaces, multispace1)
            .map(|taken_spaces: Vec<_>| {
                let flat: Vec<_> = taken_spaces.into_iter().flatten().collect();
                let shape = Array2::from_shape_vec((3, 3), flat).unwrap();

                Shape { shape }
            })
            .parse_next(input)
    }

    fn parse_present(input: &mut &str) -> winnow::Result<Shape> {
        preceded(seq!(digit1, ":", multispace1), parse_shape).parse_next(input)
    }

    fn parse_presents(input: &mut &str) -> winnow::Result<Vec<Shape>> {
        separated(0.., parse_present, multispace1).parse_next(input)
    }

    fn parse_digits(input: &mut &str) -> winnow::Result<u64> {
        digit1.try_map(str::parse::<u64>).parse_next(input)
    }

    fn parse_dimensions(input: &mut &str) -> winnow::Result<U64Vec2> {
        seq!(U64Vec2 {
            x: parse_digits,
            _: "x",
            y: parse_digits
        })
        .parse_next(input)
    }

    fn parse_present_counts(input: &mut &str) -> winnow::Result<Vec<u64>> {
        separated(0.., parse_digits, space1).parse_next(input)
    }

    fn parse_tree(input: &mut &str) -> winnow::Result<Tree> {
        seq!(
            Tree {
                dimensions: parse_dimensions,
                _: ": ",
                present_counts: parse_present_counts
            }
        )
        .parse_next(input)
    }

    fn parse_trees(input: &mut &str) -> winnow::Result<Vec<Tree>> {
        separated(0.., parse_tree, multispace1).parse_next(input)
    }

    pub fn parse_input(input: &mut &str) -> winnow::Result<Input> {
        seq!(
            Input {
                presents: parse_presents,
                _: multispace1,
                trees: parse_trees
            }
        )
        .parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use test_log::test;

    use super::*;

    const EXAMPLE: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
    const INPUT: &str = include_str!("day12_input.txt");

    #[test]
    fn test_example_input() {
        let result = puzzle(EXAMPLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_input() {
        let result = puzzle(INPUT);
        assert_eq!(result, 534);
    }

    #[test]
    #[rstest]
    #[case(
        "\
0:
###
###
###

4x5: 2
4x5: 1",
        1
    )]
    fn test_more_examples(#[case] input: &str, #[case] expected: usize) {
        let result = puzzle(input);
        assert_eq!(result, expected);
    }
}
