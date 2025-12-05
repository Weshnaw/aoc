use tracing::info;

#[derive(Debug)]
struct Accumulator {
    current_pos: i64,
    zero_count: usize,
    total_zero_hits: usize
}

impl Default for Accumulator {
    fn default() -> Self {
        Self { current_pos: 50, zero_count: 0, total_zero_hits: 0 }
    }
}

pub fn puzzle(input: &str) -> (usize, usize) {
    let count = input.trim().lines().fold(Accumulator::default(), |mut a, line| {
        if !line.is_empty() {
            let distance: usize = (&line[1..]).parse().unwrap();
            let new_pos: i64 = if line.starts_with("R") {
                a.current_pos + (distance as i64 % 100)
            } else {
                a.current_pos - (distance as i64 % 100)
            };

            if new_pos > 100 || (new_pos < 0 && a.current_pos != 0) {
                a.total_zero_hits += 1;
            }

            a.total_zero_hits += distance / 100;

            a.current_pos = new_pos.rem_euclid(100);
            if a.current_pos == 0 {
                a.zero_count += 1;
                a.total_zero_hits += 1;
            }

            info!("line: {line:>3}: {a:?}");
        }
        a
    });
    (count.zero_count, count.total_zero_hits)
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_empty_input() {
        let result = puzzle("");
        assert_eq!(result.0, 0);
    }

    #[traced_test]
    #[test]
    fn test_first_example() {
        let result = puzzle("\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
");
        assert_eq!(result, (3, 6));
    }

    #[traced_test]
    #[test]
    fn test_input() {
        let result = puzzle(include_str!("input.txt"));
        assert_eq!(result, (1150, 0));
    }
}
