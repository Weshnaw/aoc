use std::cmp::Ordering;

use bitvec::prelude::*;
use glam::Vec3;
use itertools::Itertools;
use tracing::info;

pub fn puzzle_1(input: &str, connection_count: usize) -> usize {
    let input = parse(input);

    let combinations = create_combinations(&input);

    let connections = create_connections(combinations.take(connection_count), input.len());

    info!(?connections);

    connections
        .iter()
        .map(|con| con.count_ones())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn create_combinations(input: &[Vec3]) -> impl Iterator<Item = (usize, usize)> {
    (0..input.len())
        .tuple_combinations()
        .sorted_by(|(a0, a1), (b0, b1)| {
            order_by_distance(&(&input[*a0], &input[*a1]), &(&input[*b0], &input[*b1]))
        })
}

fn create_connections(
    combinations: impl Iterator<Item = (usize, usize)>,
    input_size: usize,
) -> Vec<BitVec> {
    let mut connections: Vec<BitVec> = Vec::new();
    for (junction_a, junction_b) in combinations {
        let existing_connections: Vec<_> = connections
            .iter()
            .positions(|conn| conn[junction_a] || conn[junction_b])
            .collect();

        match existing_connections.as_slice() {
            [idx] => {
                if !(connections[*idx][junction_a] && connections[*idx][junction_b]) {
                    connections[*idx].set(junction_a, true);
                    connections[*idx].set(junction_b, true);
                }
            }
            [idx_a, idx_b] => {
                let (idx_a, idx_b) = if idx_a < idx_b {
                    (idx_a, idx_b)
                } else {
                    (idx_b, idx_a)
                };
                let conn_b = connections.remove(*idx_b);
                connections[*idx_a] |= conn_b;
            }
            _ => {
                let mut bits = bitvec!(0; input_size);
                bits.set(junction_a, true);
                bits.set(junction_b, true);
                connections.push(bits);
            }
        }
    }

    connections
}

pub fn puzzle_2(input: &str) -> f32 {
    let input = parse(input);

    let combinations = create_combinations(&input);

    let input_size = input.len();

    let mut connections: Vec<BitVec> = Vec::new();
    for (junction_a, junction_b) in combinations {
        let existing_connections: Vec<_> = connections
            .iter()
            .positions(|conn| conn[junction_a] || conn[junction_b])
            .collect();

        let junctions = match existing_connections.as_slice() {
            [idx] => {
                if !(connections[*idx][junction_a] && connections[*idx][junction_b]) {
                    connections[*idx].set(junction_a, true);
                    connections[*idx].set(junction_b, true);
                    connections[*idx].count_ones()
                } else {
                    0
                }
            }
            [idx_a, idx_b] => {
                let (idx_a, idx_b) = if idx_a < idx_b {
                    (idx_a, idx_b)
                } else {
                    (idx_b, idx_a)
                };
                let conn_b = connections.remove(*idx_b);
                connections[*idx_a] |= conn_b;
                connections[*idx_a].count_ones()
            }
            _ => {
                let mut bits = bitvec!(0; input_size);
                bits.set(junction_a, true);
                bits.set(junction_b, true);
                connections.push(bits);
                2
            }
        };

        if junctions == input_size {
            return input[junction_a].x * input[junction_b].x;
        }
    }

    0.
}

fn parse(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line
                .split(',')
                .map(|digits| digits.parse::<u64>().unwrap() as f32)
                .collect();
            Vec3::from_slice(&numbers)
        })
        .collect()
}

fn order_by_distance(a: &(&Vec3, &Vec3), b: &(&Vec3, &Vec3)) -> Ordering {
    a.0.distance_squared(*a.1)
        .partial_cmp(&b.0.distance_squared(*b.1))
        .unwrap_or(std::cmp::Ordering::Less)
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    const INPUT: &str = include_str!("day8_input.txt");

    #[test]
    fn test_puzzle_1_example_input() {
        let result = puzzle_1(EXAMPLE, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_puzzle_1_input() {
        let result = puzzle_1(INPUT, 1_000);
        assert_eq!(result, 129564);
    }

    #[test]
    fn test_puzzle_2_example_input() {
        let result = puzzle_2(EXAMPLE);
        assert_eq!(result, 25272.);
    }

    #[test]
    fn test_puzzle_2_input() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, 42047840.0);
    }
}
