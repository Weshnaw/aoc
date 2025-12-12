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

    count_from_to("you", "out", &input)
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

pub fn part2(input: &str) -> usize {
    let (_, input) = parse_input.parse_peek(input).unwrap();

    info!(?input);
    let count_dac_fft = count_from_to("dac", "fft", &input);

    // the graph cannot loop, or else the answer would be infinite
    // so we just need to know which comes first, dac or fft
    if count_dac_fft == 0 {
        // dac -> fft has 0 paths therefore fft must come first (or there are 0 paths)
        let count_svr_fft = count_from_to("svr", "fft", &input);
        let count_fft_dac = count_from_to("fft", "dac", &input);
        let count_dac_out = count_from_to("dac", "out", &input);
        count_svr_fft * count_fft_dac * count_dac_out
    } else {
        // dac comes first and we complete the calculations
        let count_svr_dac = count_from_to("svr", "dac", &input);
        let count_fft_out = count_from_to("fft", "out", &input);
        count_svr_dac * count_dac_fft * count_fft_out
    }
}

fn count_from_to(start: &str, end: &str, input: &BTreeMap<&str, Vec<&str>>) -> usize {
    let empty_vec = vec![];
    count_paths(
        &start,
        |node| input.get(*node).unwrap_or(&empty_vec),
        |node| *node == &end,
    )
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
    fn test_part1_example_input() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part1_input() {
        let result = part1(INPUT);
        assert_eq!(result, 574);
    }

    #[test]
    fn test_part2_example_input() {
        let result = part2(
            "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_input() {
        let result = part2(INPUT);
        assert_eq!(result, 306594217920240);
    }
}
