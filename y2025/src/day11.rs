use std::collections::BTreeMap;

use pathfinding::prelude::*;
use tracing::info;
use winnow::{
    Parser,
    ascii::{alpha1, multispace1, space1},
    combinator::{separated, seq},
};

pub fn part1(input: &str) -> usize {
    let (_, input) = parse_input.parse_peek(input).unwrap();

    info!(?input);

    count_paths(
        &"you",
        |node| input.get(*node).unwrap(),
        |node| *node == &"out",
    )
}

fn parse_input<'a>(input: &mut &'a str) -> winnow::Result<BTreeMap<&'a str, Vec<&'a str>>> {
    separated(
        0..,
        seq!(
            alpha1,
            _: ": ",
            separated(0.., alpha1, space1)
        ),
        multispace1,
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    const EXAMPLE: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
    const INPUT: &str = include_str!("day11_input.txt");

    #[test]
    fn test_example_input() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 5);
    }

    #[test]
    #[ignore]
    fn test_input() {
        let result = part1(INPUT);
        assert_eq!(result, 0);
    }
}
