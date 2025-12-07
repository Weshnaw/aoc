use bitvec::prelude::*;
use tracing::debug;

pub fn puzzle(input: &str) -> (usize, u64) {
    (part1(input), 0)
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| (parse_line(l), 0))
        .filter(|l| l.0.any())
        .reduce(split_tachyons)
        .unwrap_or_default()
        .1
}

fn parse_line(line: &str) -> BitVec {
    let mut tachyon_splitters = bitvec![0; line.len()];

    for idx in line
        .chars()
        .enumerate()
        .filter(|(_, c)| c != &'.')
        .map(|(idx, _)| idx)
    {
        tachyon_splitters.set(idx, true);
    }

    debug!("{line} => {tachyon_splitters:015b}");

    tachyon_splitters
}

fn split_tachyons(
    (current_beams, current_count): (BitVec, usize),
    (tachyon_splitters, _): (BitVec, usize),
) -> (BitVec, usize) {
    let splitter_hits = current_beams.clone() & &tachyon_splitters;
    let splitter_beams_removed = !splitter_hits.clone() & &current_beams;
    let mut split_left = splitter_hits.clone();
    split_left.shift_left(1);
    let mut split_right = splitter_hits.clone();
    split_right.shift_right(1);

    let result = splitter_beams_removed | split_left | split_right;

    let new_count = current_count + splitter_hits.count_ones();

    debug!(
        "{current_beams:015b} + {tachyon_splitters:015b} = {result:015b} ({current_count} => {new_count})"
    );

    (result, new_count)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use test_log::test;

    use super::*;

    #[test]
    #[rstest]
    #[case(".......S.......", bitvec!(0,0,0,0,0,0,0,1,0,0,0,0,0,0,0))]
    #[case("...............", bitvec!(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0))]
    #[case(".......^.......", bitvec!(0,0,0,0,0,0,0,1,0,0,0,0,0,0,0))]
    #[case("......^.^......", bitvec!(0,0,0,0,0,0,1,0,1,0,0,0,0,0,0))]
    #[case(".^.^.^.^.^...^.", bitvec!(0,1,0,1,0,1,0,1,0,1,0,0,0,1,0))]
    fn test_parse_line(#[case] input: &str, #[case] expected: BitVec) {
        let result = parse_line(input);
        assert_eq!(result, expected);
    }
    #[test]
    #[rstest]
    #[case(bitvec!(0,0,0,0,0,0,0,1,0,0,0,0,0,0,0), bitvec!(0,0,0,0,0,0,0,1,0,0,0,0,0,0,0), (bitvec!(0,0,0,0,0,0,1,0,1,0,0,0,0,0,0), 1))]
    #[case(bitvec!(0,0,0,0,0,0,1,0,1,0,0,0,0,0,0), bitvec!(0,0,0,0,0,0,1,0,1,0,0,0,0,0,0), (bitvec!(0,0,0,0,0,1,0,1,0,1,0,0,0,0,0), 2))]
    #[case(bitvec!(0,0,0,0,0,1,0,1,0,1,0,0,0,0,0), bitvec!(0,0,0,0,0,1,0,0,0,0,0,0,0,0,0), (bitvec!(0,0,0,0,1,0,1,1,0,1,0,0,0,0,0), 1))]
    fn test_split_tachyons(
        #[case] current: BitVec,
        #[case] splitters: BitVec,
        #[case] expected: (BitVec, usize),
    ) {
        let result = split_tachyons((current, 0), (splitters, 0));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_example_input() {
        let result = puzzle(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        assert_eq!(result, (21, 0));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day7_input.txt"));
        assert_eq!(result, (1539, 0));
    }
}
