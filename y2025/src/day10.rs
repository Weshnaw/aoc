use crate::day10::parsing::parse_full_input;
use pathfinding::prelude::*;
use rayon::prelude::*;
use tracing::info;
use winnow::Parser;
use z3::{Optimize, ast::Int};

#[derive(Debug, PartialEq)]
struct Machine {
    desired_state: u64,
    button_masks: Vec<u64>,
    joltage_requirements: Vec<u64>,
}

pub fn part1(input: &str) -> u64 {
    let (left_over, input) = parse_full_input.parse_peek(input).unwrap();

    info!(?left_over);

    input.par_iter().map(part1_solve_single_machine).sum()
}

fn part1_solve_single_machine(machine: &Machine) -> u64 {
    let res = dijkstra(
        &0u64,
        |state| press_buttons_part1(*state, &machine.button_masks),
        |state| state == &machine.desired_state,
    )
    .unwrap();

    res.1 as u64
}

fn press_buttons_part1(state: u64, buttons: &[u64]) -> impl Iterator<Item = (u64, u32)> {
    buttons
        .iter()
        .map(move |button| (push_button_part1(state, *button), 1))
}

fn push_button_part1(state: u64, button_mask: u64) -> u64 {
    state ^ button_mask
}

pub fn part2(input: &str) -> u64 {
    let (left_over, input) = parse_full_input.parse_peek(input).unwrap();

    info!(?left_over);

    input.par_iter().map(part2_solve_single_machine).sum()
}

fn part2_solve_single_machine(machine: &Machine) -> u64 {
    let optimizer = Optimize::new();

    let target: Vec<_> = machine
        .joltage_requirements
        .iter()
        .map(|joltage| Int::from_u64(*joltage))
        .collect();

    let dim = target.len();
    let available_actions: Vec<Vec<u64>> = machine
        .button_masks
        .iter()
        .map(|button| {
            let mut action = vec![0; dim];
            for idx in BitIter(*button) {
                action[idx] = 1;
            }
            action
        })
        .collect();

    let actions_taken: Vec<_> = (0..available_actions.len())
        .map(|action| {
            let name = format!("x{}", action);
            let vars = Int::new_const(name);
            // constrains steps so there can't be a negative number of actions taken for a given button
            optimizer.assert(&vars.ge(Int::from_u64(0)));
            vars
        })
        .collect();

    // sets the constraint that the current state >= target state
    target.iter().enumerate().for_each(|(joltage_idx, target)| {
        // calculates the joltage for a given idx by suming up all the previous actions taken
        let current = available_actions.iter().enumerate().fold(
            Int::from_u64(0),
            |acc, (action_idx, action)| {
                acc + &actions_taken[action_idx] * &Int::from_u64(action[joltage_idx])
            },
        );

        optimizer.assert(&current.ge(target));
    });

    // sums the individual action counts
    let total_actions_taken = actions_taken
        .iter()
        .fold(Int::from_u64(0), |acc, action_count| &acc + action_count);
    optimizer.minimize(&total_actions_taken);

    match optimizer.check(&[]) {
        z3::SatResult::Sat => {
            let model = optimizer.get_model().unwrap();
            let result = model.eval(&total_actions_taken, true).unwrap();
            result.as_u64().unwrap()
        }
        _ => u64::MAX,
    }
}
struct BitIter(u64);

impl Iterator for BitIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let index = self.0.trailing_zeros();
        self.0 ^= 1 << index;
        Some(index as usize)
    }
}

mod parsing {
    use super::*;

    use winnow::{
        ascii::{digit1, line_ending, space0, space1},
        combinator::{alt, delimited, repeat, separated, seq},
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
        separated(0.., parse_line, line_ending).parse_next(input)
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
    fn test_part1_example_input() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part1_input() {
        let result = part1(INPUT);
        assert_eq!(result, 486);
    }

    #[test]
    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 2)]
    fn test_part1_solve_single_machine(#[case] input: &str, #[case] expected: u64) {
        use crate::day10::parsing::parse_line;

        let (_, machine) = parse_line.parse_peek(input).unwrap();

        let result = part1_solve_single_machine(&machine);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_example_input() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 33);
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let result = part2(INPUT);
        assert_eq!(result, 17820);
    }

    #[test]
    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 101)]
    fn test_part2_solve_single_machine(#[case] input: &str, #[case] expected: u64) {
        use crate::day10::parsing::parse_line;

        let (_, machine) = parse_line.parse_peek(input).unwrap();

        let result = part2_solve_single_machine(&machine);

        assert_eq!(result, expected);
    }
}
