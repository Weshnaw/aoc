use crate::day10::parsing::parse_full_input;
use rayon::prelude::*;
use winnow::Parser;

#[derive(Debug, PartialEq)]
struct Machine {
    current_state: u64,
    desired_state: u64,
    button_masks: Vec<u64>,
    joltage_requirements: Vec<u64>,
}

pub fn puzzle(input: &str) -> (u64, u64) {
    let input = parse_full_input.parse(input).unwrap();

    (input.par_iter().map(solve_single_machine).sum(), 0)
}

fn solve_single_machine(_input: &Machine) -> u64 {
    todo!()
}

mod parsing {
    use super::*;

    use winnow::{
        ascii::{digit1, newline, space0, space1},
        combinator::{alt, delimited, empty, repeat, separated, seq},
    };

    fn parse_single_light(input: &mut &str) -> winnow::Result<bool> {
        alt((".", "#")).map(|char| char == "#").parse_next(input)
    }

    fn parse_multiple_lights(input: &mut &str) -> winnow::Result<u64> {
        repeat(0.., parse_single_light)
            .map(|lights: Vec<bool>| {
                lights
                    .iter()
                    .rev()
                    .fold(0u64, |acc, light| (acc << 1) + *light as u64)
            })
            .parse_next(input)
    }

    fn parse_desired_state(input: &mut &str) -> winnow::Result<u64> {
        delimited("[", parse_multiple_lights, "]").parse_next(input)
    }

    fn parse_cs_digits(input: &mut &str) -> winnow::Result<Vec<u64>> {
        separated(0.., digit1.try_map(str::parse::<u64>), ",").parse_next(input)
    }

    fn parse_single_button(input: &mut &str) -> winnow::Result<u64> {
        delimited(
            "(",
            parse_cs_digits.map(|numbers: Vec<u64>| {
                numbers.iter().fold(0u64, |acc, digit| acc | (1 << digit))
            }),
            ")",
        )
        .parse_next(input)
    }

    fn parse_button_masks(input: &mut &str) -> winnow::Result<Vec<u64>> {
        separated(0.., parse_single_button, space1).parse_next(input)
    }

    fn parse_joltage_requirements(input: &mut &str) -> winnow::Result<Vec<u64>> {
        delimited("{", parse_cs_digits, "}").parse_next(input)
    }

    pub fn parse_line(input: &mut &str) -> winnow::Result<Machine> {
        seq!(
            Machine {
                current_state: empty.value(0u64),
                desired_state: parse_desired_state,
                _: space0,
                button_masks: parse_button_masks,
                _: space0,
                joltage_requirements: parse_joltage_requirements
            }
        )
        .parse_next(input)
    }

    pub fn parse_full_input(input: &mut &str) -> winnow::Result<Vec<Machine>> {
        separated(0.., parse_line, newline).parse_next(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use rstest::rstest;
        use test_log::test;
        use tracing::info;

        #[test]
        #[rstest]
        #[case("[.##.]", 0b0110)]
        #[case("[...#.]", 0b01000)]
        #[case("[.###.#]", 0b101110)]
        fn test_parse_desired_state(#[case] input: &str, #[case] expected: u64) {
            let (left_over, result) = parse_desired_state.parse_peek(input).unwrap();

            assert_eq!(result, expected);
            assert!(left_over.is_empty());
        }

        #[test]
        #[rstest]
        #[case("(3)", 0b1000)]
        #[case("(1,2)", 0b110)]
        #[case("(0,1)", 0b11)]
        #[case("(0,2,3,4)", 0b11101)]
        fn test_single_button(#[case] input: &str, #[case] expected: u64) {
            let (left_over, result) = parse_single_button.parse_peek(input).unwrap();
            info!("{expected:0b}");
            info!("{result:0b}");
            assert_eq!(result, expected);
            assert!(left_over.is_empty());
        }

        #[test]
        #[rstest]
        #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", Machine {
            current_state: 0,
            desired_state: 0b0110,
            button_masks: vec![0b1000, 0b1010, 0b100, 0b1100, 0b101, 0b11],
            joltage_requirements: vec![3, 5, 4, 7]
         })]
        fn test_parse_line(#[case] input: &str, #[case] expected: Machine) {
            let (left_over, result) = parse_line.parse_peek(input).unwrap();
            info!(?expected, ?result);
            assert_eq!(result, expected);
            assert!(left_over.is_empty());
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use test_log::test;

    use super::*;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    const INPUT: &str = include_str!("day10_input.txt");

    #[test]
    fn test_example_input() {
        let result = puzzle(EXAMPLE);
        assert_eq!(result, (7, 0));
    }

    #[test]
    fn test_input() {
        let result = puzzle(INPUT);
        assert_eq!(result, (0, 0));
    }

    #[test]
    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 2)]
    fn test_solve(#[case] input: &str, #[case] expected: u64) {
        use crate::day10::parsing::parse_line;

        let (_, machine) = parse_line.parse_peek(input).unwrap();

        let result = solve_single_machine(&machine);

        assert_eq!(result, expected);
    }
}
