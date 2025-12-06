use tracing::info;

pub fn puzzle(input: &str) -> (u64, u64) {
    let input = input.trim();
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

fn reduce_via_operators(mut acc: Vec<u64>, nums: Vec<u64>, operators: &Vec<&str>) -> Vec<u64> {
    for (idx, val) in acc.iter_mut().enumerate() {
        match operators[idx] {
            "*" => *val *= nums[idx],
            _ => *val += nums[idx],
        }
    }
    acc
}

fn solve_part2(input: &str) -> u64 {
    let mut lines = input.lines().rev();

    let operators: Vec<&str> = lines.next().unwrap().split_ascii_whitespace().collect();

    let parsed_digits: Vec<_> = lines.map(parse_line).collect();

    let parsed_digits = transpose(parsed_digits);

    info!(?parsed_digits);

    parsed_digits
        .iter()
        .map(map_digits)
        .reduce(|acc, nums| reduce_via_operators(acc, nums, &operators))
        .unwrap_or_default()
        .iter()
        .sum()
}

fn parse_line(input: &str) -> Vec<Vec<u64>> {
    input
        .split_ascii_whitespace()
        .map(|digits| {
            digits
                .as_bytes()
                .iter()
                .map(|digit| (digit & 0x0F) as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn transpose<T: Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = input.len();
    let cols = input[0].len();

    let mut transposed = vec![Vec::with_capacity(rows); cols];

    for row in 0..rows {
        for col in 0..cols {
            transposed[col].push(input[row][col].clone());
        }
    }
    transposed
}

fn map_digits(numbers: &Vec<Vec<u64>>) -> Vec<u64> {
    let transposed = transpose_with_padding(numbers.clone());
    info!(?transposed);

    transposed
        .into_iter()
        .map(|digits| {
            let mut result: u64 = 0;
            for digit in digits.into_iter().filter_map(|d| d) {
                result = (result * 10) + digit
            }
            result
        })
        .collect()
}

fn transpose_with_padding<T: Clone>(_input: Vec<Vec<T>>) -> Vec<Vec<Option<T>>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use test_log::test;

    use super::*;

    #[test]
    #[rstest]
    #[case("123 328 51 64", vec![vec![1, 2, 3], vec![3, 2, 8], vec![5, 1], vec![6, 4]])]
    #[case("45 64  387 23", vec![vec![4, 5], vec![6, 4], vec![3, 8, 7], vec![2, 3]])]
    #[case("6 98  215 314", vec![vec![6], vec![9, 8], vec![2, 1, 5], vec![3, 1, 4]])]
    fn test_parse_line(#[case] input: &str, #[case] expected: Vec<Vec<u64>>) {
        let result = parse_line(input);

        assert_eq!(result, expected);
    }
    
    #[test]
    #[rstest]
    #[case(vec![
                vec![vec![1, 2, 3], vec![3, 2, 8], vec![5, 1]   , vec![6, 4]   ], 
                vec![vec![4, 5]   , vec![6, 4]   , vec![3, 8, 7], vec![2, 3]   ],
                vec![vec![6]      , vec![9, 8]   , vec![2, 1, 5], vec![3, 1, 4]]
            ], 
           vec![
                vec![vec![1, 2, 3], vec![4, 5]   , vec![6]      ],
                vec![vec![3, 2, 8], vec![6, 4]   , vec![9, 8]   ],
                vec![vec![5, 1]   , vec![3, 8, 7], vec![2, 1, 5]],
                vec![vec![6, 4]   , vec![2, 3]   , vec![3, 1, 4]],
            ])]
    fn test_transpose(#[case] input: Vec<Vec<Vec<u64>>>, #[case] expected: Vec<Vec<Vec<u64>>>) {
        let result = transpose(input);

        assert_eq!(result, expected);
    }

    #[test]
    #[rstest]
    #[case(vec![vec![1, 2, 3], vec![4, 5]   , vec![6]      ], vec![  1,  24, 356])]
    #[case(vec![vec![3, 2, 8], vec![6, 4]   , vec![9, 8]   ], vec![  8, 248, 369])]
    #[case(vec![vec![5, 1]   , vec![3, 8, 7], vec![2, 1, 5]], vec![175, 581,  32])]
    #[case(vec![vec![6, 4]   , vec![2, 3]   , vec![3, 1, 4]], vec![  4, 431, 623])]
    // #[case(vec![vec![], vec![], vec![]], vec![])]
    fn test_map_digits(#[case] input: Vec<Vec<u64>>, #[case] mut expected: Vec<u64>) {
        let mut result = map_digits(&input);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    #[rstest]
    #[case(vec![
        vec![1, 2, 3], 
        vec![   4, 5], 
        vec![      6]
    ], vec![
        vec![None   , None   , Some(1)], 
        vec![None   , Some(2), Some(4)], 
        vec![Some(3), Some(5), Some(6)]
    ])]
    #[case(vec![
        vec![3, 2, 8], 
        vec![   6, 4], 
        vec![   9, 8]   
    ], vec![
        vec![None   , None   , Some(8)], 
        vec![Some(2), Some(4), Some(8)], 
        vec![Some(3), Some(6), Some(9)]
    ])]
    #[case(vec![
        vec![1, 2, 3], 
        vec![   4, 5], 
        vec![      6]
    ], vec![
        vec![None   , None   , Some(1)], 
        vec![None   , Some(2), Some(4)], 
        vec![Some(3), Some(5), Some(6)]
    ])]
    #[case(vec![
        vec![1, 2, 3], 
        vec![   4, 5], 
        vec![      6]
    ], vec![
        vec![None   , None   , Some(1)], 
        vec![None   , Some(2), Some(4)], 
        vec![Some(3), Some(5), Some(6)]
    ])]
    fn test_transpose_with_padding(#[case] input: Vec<Vec<u64>>, #[case] mut expected: Vec<Vec<Option<u64>>>) {
        let mut result = transpose_with_padding(input);

        result.sort();
        expected.sort();

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
    fn test_input() {
        let result = puzzle(include_str!("day6_input.txt"));
        assert_eq!(result, (4719804927602, 0));
    }
}
