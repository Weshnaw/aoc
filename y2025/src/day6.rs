pub fn puzzle(input: &str) -> (u64, u64) {
    let input = input.trim();
    if input.is_empty() {
        return (0, 0);
    }

    let mut lines = input.lines().rev();

    let operators: Vec<&str> = lines.next().unwrap().split_ascii_whitespace().collect();

    let part1 = lines
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|num| num.parse().unwrap_or_default())
                .collect::<Vec<u64>>()
        })
        .reduce(|mut acc, nums| {
            for (idx, val) in acc.iter_mut().enumerate() {
                match operators[idx] {
                    "*" => *val *= nums[idx],
                    _ => *val += nums[idx],
                }
            }
            acc
        })
        .unwrap_or_default()
        .iter()
        .sum();

    (part1, 0)
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
