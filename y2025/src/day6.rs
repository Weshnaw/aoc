use tracing::{debug, info};

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
    let mut lines = input.lines().rev();

    let operator_line = lines.next().unwrap();

    let operators: Vec<&str> = operator_line.split_ascii_whitespace().collect();
    let number_lengths: Vec<usize> = count_whitespaces(operator_line);

    info!(?operators, ?number_lengths, "operators");

    let parsed_digits: Vec<_> = lines.map(|l| parse_line(l, &number_lengths)).collect();

    let parsed_digits = transpose(parsed_digits);

    info!(?parsed_digits, "first transpose");

    let parsed_digits: Vec<_> = parsed_digits.iter().map(map_digits).collect();

    info!(?parsed_digits, "mapped digits");

    parsed_digits
        .into_iter()
        .enumerate()
        .map(|(idx, numbers)| match operators[idx] {
            "*" => numbers
                .into_iter()
                .reduce(|acc, n| n * acc)
                .unwrap_or_default(),
            _ => numbers.into_iter().sum::<u64>(),
        })
        .sum()
}

fn count_whitespaces(input: &str) -> Vec<usize> {
    let mut whitespace_counts = Vec::new();
    let mut count = 0;

    for char in input.chars() {
        if char.is_whitespace() {
            count += 1;
        } else if count > 0 {
            whitespace_counts.push(count);
            count = 0;
        }
    }

    if count > 0 {
        whitespace_counts.push(count + 1);
    }

    whitespace_counts
}

fn parse_line(line: &str, num_lengths: &Vec<usize>) -> Vec<Vec<Option<u64>>> {
    let mut counter = 0;
    let chars = line.as_bytes();
    let mut result = Vec::new();
    for length in num_lengths {
        let mut digits: Vec<Option<u64>> = Vec::with_capacity(*length);
        for char in chars[counter..counter + length].iter() {
            if char.is_ascii_whitespace() {
                digits.push(None);
            } else {
                digits.push(Some((char & 0x0F) as u64));
            }
        }

        counter += 1 + length;
        result.push(digits);
    }

    debug!(?result);

    assert_eq!(result.len(), num_lengths.len());

    result
}

fn transpose<T: Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = input.len();
    let cols = input[0].len();

    let mut transposed = vec![Vec::with_capacity(rows); cols];

    for input_row in input.iter().take(rows) {
        for (col, input_item) in input_row.iter().take(cols).enumerate() {
            transposed[col].push(input_item.clone());
        }
    }
    transposed
}

fn map_digits(numbers: &Vec<Vec<Option<u64>>>) -> Vec<u64> {
    let transposed = transpose(numbers.clone());
    debug!(?numbers, ?transposed);

    let result: Vec<_> = transposed
        .into_iter()
        .map(|digits| {
            let mut result: u64 = 0;
            for digit in digits.into_iter().flatten().rev() {
                result = (result * 10) + digit
            }
            result
        })
        .collect();

    debug!(?result);

    result
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use test_log::test;

    use super::*;

    #[test]
    #[rstest]
    #[case("*   +   *   +  ", vec![3, 3, 3, 3])]
    fn test_count_whitespaces(#[case] input: &str, #[case] expected: Vec<usize>) {
        let result = count_whitespaces(input);

        assert_eq!(result, expected);
    }

    #[test]
    #[rstest]
    #[case("123 328  51 64 ", vec![3, 3, 3, 3],
        vec![vec![Some(1), Some(2), Some(3)],
             vec![Some(3), Some(2), Some(8)],
             vec![None,    Some(5), Some(1)],
             vec![Some(6), Some(4), None   ]])]
    #[case(" 45 64  387 23 ", vec![3, 3, 3, 3],
        vec![vec![None,    Some(4), Some(5)],
             vec![Some(6), Some(4), None   ],
             vec![Some(3), Some(8), Some(7)],
             vec![Some(2), Some(3), None   ]])]
    #[case("  6 98  215 314", vec![3, 3, 3, 3],
        vec![vec![None,    None,    Some(6)],
             vec![Some(9), Some(8), None   ],
             vec![Some(2), Some(1), Some(5)],
             vec![Some(3), Some(1), Some(4)]])]
    fn test_parse_line(
        #[case] input: &str,
        #[case] num_sizes: Vec<usize>,
        #[case] expected: Vec<Vec<Option<u64>>>,
    ) {
        let result = parse_line(input, &num_sizes);

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
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(result, (4277556, 3263827));
    }

    #[test]
    fn test_example2_input() {
        let result = puzzle(
            "123 328  51 64  23
 45 64  387 23   3
  6 98  215 314 23
*   +   *   +   + ",
        );
        assert_eq!(result, (4277605, 3264182));
    }

    #[test]
    fn test_example3_input() {
        let result = puzzle(
            "123 328  51 64  23
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
