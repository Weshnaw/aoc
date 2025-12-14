use glam::U64Vec2;
use ndarray::Array2;
use tracing::debug;
use winnow::Parser;
use z3::{
    Solver,
    ast::{Bool, Int},
};

use crate::day12::parsing::parse_input;

pub fn puzzle(input: &str) -> usize {
    let (_, input) = parse_input.parse_peek(input).unwrap();

    input
        .trees
        .iter()
        .filter(|tree| tree.is_possible(&input.presents))
        .count()
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
    fn area_of_3x3s(&self) -> u64 {
        (self.dimensions.x / 3) * (self.dimensions.y / 3)
    }

    fn is_possible(&self, presents: &[Shape]) -> bool {
        let total_present_area = self
            .present_counts
            .iter()
            .enumerate()
            .map(|(idx, count)| count * presents[idx].area())
            .sum::<u64>();
        if self.area() < total_present_area {
            // trivially impossible
            return false;
        }

        if self.area_of_3x3s() >= self.present_counts.iter().sum::<u64>() {
            // trivially possible via sparse filling
            return true;
        }

        debug!(?self);

        z3_solver(presents, &self.dimensions, &self.present_counts)
    }
}

fn z3_solver(presents: &[Shape], dims: &U64Vec2, desired_presents: &[u64]) -> bool {
    let solver = Solver::new();

    let dims = (Int::from_u64(dims.x), Int::from_u64(dims.y));

    let presents: Vec<_> = presents
        .iter()
        .map(|present| {
            let shape: Vec<_> = present
                .shape
                .iter()
                .map(|space| Bool::from_bool(*space))
                .collect();

            Array2::from_shape_vec((3, 3), shape).unwrap()
        })
        .collect();

    let desired: Vec<_> = desired_presents
        .iter()
        .enumerate()
        .flat_map(|(idx, desired)| create_z3_consts(&solver, idx, *desired, &dims))
        .collect();

    let mut occupied_positions: Vec<(Int, Int, &Bool)> = vec![];

    for (present_idx, (position_x, position_y), _rotation, _mirrored) in desired {
        let current_present = &presents[present_idx];

        for x in 0..3 {
            for y in 0..3 {
                // TODO: somehow account for mirroring and rotations
                let position_x = Int::from_u64(x) + &position_x;
                let position_y = Int::from_u64(y) + &position_y;

                let x_in_bounds =
                    Bool::and(&[position_x.ge(Int::from_u64(0)), position_x.lt(&dims.0)]);
                let y_in_bounds =
                    Bool::and(&[position_y.ge(Int::from_u64(0)), position_y.lt(&dims.1)]);

                let space_desired = &current_present[(x as usize, y as usize)];

                for occupied in occupied_positions.iter() {
                    solver.assert(Bool::or(&[
                        space_desired.not(),
                        Bool::or(&[
                            x_in_bounds.not(),
                            y_in_bounds.not(),
                            Bool::and(&[
                                &position_x.eq(&occupied.0),
                                &position_y.eq(&occupied.1),
                                occupied.2,
                                &x_in_bounds,
                                &y_in_bounds,
                            ]),
                        ]),
                    ]));
                }

                occupied_positions.push((position_x, position_y, space_desired));
            }
        }
    }

    matches!(solver.check(), z3::SatResult::Sat)
}

fn create_z3_consts(
    solver: &Solver,
    present_idx: usize,
    desired: u64,
    (x_limit, y_limit): &(Int, Int),
) -> Vec<(usize, (Int, Int), Int, Bool)> {
    (0..desired)
        .map(|p| {
            let pos_name = format!("x_position_{present_idx}_{p}");
            let x_position = Int::new_const(pos_name);
            solver.assert(x_position.ge(Int::from_u64(0)));
            solver.assert(x_position.lt(x_limit));
            let pos_name = format!("y_position_{present_idx}_{p}");
            let y_position = Int::new_const(pos_name);
            solver.assert(y_position.ge(Int::from_u64(0)));
            solver.assert(y_position.lt(y_limit));
            let position = (x_position, y_position);

            let rot_name = format!("rotation_{present_idx}_{p}");
            let rotation = Int::new_const(rot_name);
            solver.assert(rotation.ge(Int::from_u64(0)));
            solver.assert(rotation.lt(Int::from_u64(4)));

            let mir_name = format!("mirrored_{present_idx}_{p}");
            let mirrored = Bool::new_const(mir_name);

            (present_idx, position, rotation, mirrored)
        })
        .collect()
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
    #[case(
        "\
0:
###
#..
###

4x4: 2",
        1
    )]
    fn test_more_examples(#[case] input: &str, #[case] expected: usize) {
        let result = puzzle(input);
        assert_eq!(result, expected);
    }
}
