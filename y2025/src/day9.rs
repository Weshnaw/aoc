use glam::I64Vec2;
use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    let red_tiles = parse(input);

    red_tiles
        .iter()
        .tuple_combinations()
        .map(calc_area)
        .max()
        .unwrap_or_default()
}

fn parse(input: &str) -> Vec<I64Vec2> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            I64Vec2::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn calc_area((a, b): (&I64Vec2, &I64Vec2)) -> i64 {
    let diff = (a.max(*b) - a.min(*b)) + 1;
    diff.x * diff.y
}

pub fn part2(input: &str) -> i64 {
    let red_tiles = parse(input);

    let lines: Vec<(_, _)> = red_tiles.iter().circular_tuple_windows().collect();

    red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| ((a, b), calc_area((a, b))))
        .sorted_by_key(|v| v.1)
        .rev()
        .find(|(a, _)| exists_in_tiles(*a, &lines))
        .map(|f| f.1)
        .unwrap_or_default()
}

fn exists_in_tiles(tiles: (&I64Vec2, &I64Vec2), lines: &[(&I64Vec2, &I64Vec2)]) -> bool {
    lines.iter().all(|line| check_rectangle(tiles, *line))
}

fn check_rectangle((a, b): (&I64Vec2, &I64Vec2), (start, end): (&I64Vec2, &I64Vec2)) -> bool {
    let max_tile = a.max(*b);
    let min_tile = a.min(*b);
    let max_line = start.max(*end);
    let min_line = start.min(*end);

    let left = max_tile.x <= min_line.x;
    let right = min_tile.x >= max_line.x;
    let top = max_tile.y <= min_line.y;
    let bottom = min_tile.y >= max_line.y;

    left || right || top || bottom
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    const INPUT: &str = include_str!("day9_input.txt");

    #[test]
    fn test_part1_example_input() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part1_input() {
        let result = part1(INPUT);
        assert_eq!(result, 4758121828);
    }
    #[test]
    fn test_part2_example_input() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part2_input() {
        let result = part2(INPUT);
        assert_eq!(result, 1577956170);
    }
}
