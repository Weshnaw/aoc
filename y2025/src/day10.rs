
struct Lights {
    current_state: u64,
    desired_state: u64,
    button_masks: Vec<u64>,
    joltage_requirements: Vec<u64>
}

// part1 TODO;
// 1. parse the desired state into a binary i.e. [.##.] = 0110
// 2. parse the buttons into a list of masks i.e. (0,2) = 101
// 3. parse the joltage_requirements just as the numbers (for now)
// note: you can toggle a button by xor'ing the state with the mask
// 4. run through the possibilities, likely keeping some cache of visited states, if a state is visited we can cull that path
// 5. sum the minimum found press count

pub fn puzzle(_input: &str) -> (u64, u64) {
    todo!("puzzle");
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
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
}
