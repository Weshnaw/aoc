use ndarray::{Array2, array};
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};
use tracing::{debug, info};

pub fn puzzle(input: &str) -> (usize, usize) {
    if input.is_empty() {
        return (0, 0);
    }

    let mut input = transform_str_to_ndarray(input);
    info!("{input:?}");

    let part1_result = remove_paper(&mut input);

    let mut removed = part1_result;
    let mut total = removed;
    while removed > 0 {
        removed = remove_paper(&mut input);
        total += removed;
    }

    (part1_result, total)
}

fn remove_paper(input: &mut Array2<u8>) -> usize {
    let kernel = array![[1u8, 1u8, 1u8], [1u8, 0u8, 1u8], [1u8, 1u8, 1u8]];
    let sums = input
        .conv(&kernel, ConvMode::Same, PaddingMode::Zeros)
        .unwrap();

    debug!("{sums:?}");

    input
        .iter_mut()
        .zip(sums.iter())
        .filter_map(|(input, conv)| {
            if (input == &1) && (conv < &4) {
                *input = 0;
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn transform_str_to_ndarray(input: &str) -> Array2<u8> {
    let raw_vec = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '@' => 1,
                    _ => 0,
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    Array2::from_shape_vec(
        (raw_vec.len(), raw_vec[0].len()),
        raw_vec.into_iter().flatten().collect(),
    )
    .unwrap()
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
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        assert_eq!(result, (13, 43));
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day4_input.txt"));
        assert_eq!(result, (1435, 8623));
    }
}
