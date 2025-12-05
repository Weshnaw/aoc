use ndarray::{Array2, array};
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};
use tracing::info;

pub fn puzzle(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }

    let input = transform_str_to_ndarray(input);
    info!("{input:?}");

    let kernel = array![[
        1u8, 1u8, 1u8
    ],[
        1u8, 0u8, 1u8
    ],[
        1u8, 1u8, 1u8
    ]];

    let sums = input.conv(&kernel, ConvMode::Same, PaddingMode::Zeros).unwrap();
    info!("{sums:?}");

    let result = input.iter().zip(sums.iter()).filter(|(input, conv)| (input == &&1) && (conv < &&4)).count();

    result
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
        assert_eq!(result, 0);
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
        assert_eq!(result, 13);
    }

    #[test]
    fn test_input() {
        let result = puzzle(include_str!("day4_input.txt"));
        assert_eq!(result, 1435);
    }
}
