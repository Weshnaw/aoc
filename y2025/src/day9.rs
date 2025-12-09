use glam::U64Vec2;
use itertools::Itertools;

pub fn puzzle(input: &str) -> (u64, u64) {
    let red_tiles = parse(input);

    let max_area = red_tiles
        .iter()
        .tuple_combinations()
        .map(calc_area)
        .max()
        .unwrap_or_default();

    (max_area, 0)
}

fn parse(input: &str) -> Vec<U64Vec2> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            U64Vec2::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn calc_area((a, b): (&U64Vec2, &U64Vec2)) -> u64 {
    let diff = (a.max(*b) - a.min(*b)) + 1;
    diff.x * diff.y
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_example_input() {
        let result = puzzle(
            "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(result, (50, 0));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day9_input.txt"));
        assert_eq!(result, (4758121828, 0));
    }
}
