use ndarray::{Array2, array};
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};
use tracing::{debug, info};

pub fn puzzle(input: &str) -> (usize, usize) {
    if input.is_empty() {
        return (0, 0);
    }

    let mut input = transform_str_to_ndarray(input);
    info!("{input:?}");

    let kernel = array![[1u8, 1u8, 1u8], [1u8, 0u8, 1u8], [1u8, 1u8, 1u8]];

    let part1_result = remove_paper(&mut input, &kernel);

    let mut removed = part1_result;
    let mut total = removed;
    while removed > 0 {
        removed = remove_paper(&mut input, &kernel);
        total += removed;
    }

    (part1_result, total)
}

fn remove_paper(input: &mut Array2<u8>, kernal: &Array2<u8>) -> usize {
    let sums = input
        .conv(kernal, ConvMode::Same, PaddingMode::Zeros)
        .unwrap();

    debug!("{sums:?}");

    let mut removed = 0;
    for (input, conv) in input.iter_mut().zip(sums.iter()) {
        if *input == 1 && *conv < 4 {
            *input = 0;
            removed += 1;
        }
    }

    removed
}

fn transform_str_to_ndarray(input: &str) -> Array2<u8> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let mut data = Vec::with_capacity(rows * cols);

    for line in input.lines() {
        for c in line.bytes() {
            data.push((c == b'@') as u8);
        }
    }

    Array2::from_shape_vec((rows, cols), data).unwrap()
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
