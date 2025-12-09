use glam::U64Vec2;
use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    let red_tiles = parse(input);

    red_tiles
        .iter()
        .tuple_combinations()
        .map(calc_area)
        .max()
        .unwrap_or_default()
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

fn calc_area((corner_a, corner_b): (&U64Vec2, &U64Vec2)) -> u64 {
    let diff = (corner_a.max(*corner_b) - corner_a.min(*corner_b)) + 1;
    diff.x * diff.y
}

pub fn part2(input: &str) -> u64 {
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

// Note: This makes a few assumptions about the ouput, so could fail a _technically_ possible edge case
// a more perfect implementation would do additional intersection tests with the edges and lines
fn exists_in_tiles(corner_tiles: (&U64Vec2, &U64Vec2), lines: &[(&U64Vec2, &U64Vec2)]) -> bool {
    lines
        .iter()
        .all(|line| bounding_box_check(corner_tiles, *line))
}

fn bounding_box_check(
    (tile_a, tile_b): (&U64Vec2, &U64Vec2),
    (line_start, line_end): (&U64Vec2, &U64Vec2),
) -> bool {
    let max_tile_coords = tile_a.max(*tile_b);
    let min_tile_coords = tile_a.min(*tile_b);
    let max_line_coords = line_start.max(*line_end);
    let min_line_coords = line_start.min(*line_end);

    let left = max_tile_coords.x <= min_line_coords.x;
    let right = min_tile_coords.x >= max_line_coords.x;
    let top = max_tile_coords.y <= min_line_coords.y;
    let bottom = min_tile_coords.y >= max_line_coords.y;

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
