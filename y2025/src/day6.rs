use tracing::debug;

pub fn puzzle(input: &str) -> (u64, u64) {
    if input.is_empty() {
        return (0, 0);
    }

    (solve_part1(input), solve_part2(input))
}

// these problems are different enough at least when looking at my part 1 solution that I am creating two distinct functions

fn solve_part1(input: &str) -> u64 {
    let mut lines = input.lines().rev();

    let operators: Vec<&str> = lines.next().unwrap().split_ascii_whitespace().collect();

    lines
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|num| num.parse().unwrap_or_default())
                .collect::<Vec<u64>>()
        })
        .reduce(|acc, nums| reduce_via_operators(acc, nums, &operators))
        .unwrap_or_default()
        .iter()
        .sum()
}

fn reduce_via_operators(mut acc: Vec<u64>, nums: Vec<u64>, operators: &[&str]) -> Vec<u64> {
    for (idx, val) in acc.iter_mut().enumerate() {
        match operators[idx] {
            "*" => *val *= nums.get(idx).unwrap_or(&1),
            _ => *val += nums.get(idx).unwrap_or(&0),
        }
    }
    acc
}

fn solve_part2(input: &str) -> u64 {
    let operator_line = input.lines().last().unwrap();
    let operators: Vec<_> = operator_line.split_ascii_whitespace().collect();
    let width = operator_line.len();

    let input = &input[..(input.len() - width)].replace(['\n', '\r'], "");
    let height = input.len() / width;

    debug!(?input, ?height, ?width);
    let mut transposed = vec![0u8; input.len()];
    transpose::transpose(input.as_bytes(), &mut transposed, width, height);

    let transposed: Vec<_> = transposed
        .chunks(height)
        .map(|digits| str::from_utf8(digits).unwrap().replace(' ', ""))
        .collect::<Vec<_>>();
    let transposed: Vec<_> = transposed.split(|digits| digits.is_empty()).collect();

    debug!(?transposed);

    operators
        .iter()
        .enumerate()
        .map(|(idx, &op)| {
            let nums = transposed[idx]
                .iter()
                .map(|num| num.parse::<u64>().unwrap());
            match op {
                "*" => nums.product::<u64>(),
                _ => nums.sum::<u64>(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_example_input() {
        let result = puzzle(
            "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(result, (4277556, 3263827));
    }

    #[test]
    fn test_example2_input() {
        let result = puzzle(
            "\
123 328  51 64  23
 45 64  387 23   3
  6 98  215 314 23
*   +   *   +   + ",
        );
        assert_eq!(result, (4277605, 3264182));
    }

    #[test]
    fn test_example3_input() {
        let result = puzzle(
            "\
123 328  51 64  23
 45 64  387 23   3
  6 98  215 314 23
*   +   *   +   * ",
        );
        assert_eq!(result, (4279143, 3271153));
    }

    #[test]
    fn test_example4_input() {
        let result = puzzle(
            "\
23 328  51 64 
45 64  387 23 
 6 98  215 314
*  +   *   +  ",
        );
        assert_eq!(result, (4250556, 3263827));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day6_input.txt"));
        assert_eq!(result, (4719804927602, 9608327000261));
    }
}
