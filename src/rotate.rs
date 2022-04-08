/// You are given an n x n 2D matrix representing an image,
/// rotate the image by 90 degrees (clockwise).
///
/// You have to rotate the image in-place,
/// which means you have to modify the input 2D matrix directly.
/// DO NOT allocate another 2D matrix and do the rotation.
///
/// Input: matrix = [
///     [1,2,3],
///     [4,5,6],
///     [7,8,9]
/// ]
///
/// Output: [
///     [7,4,1],
///     [8,5,2],
///     [9,6,3]
/// ]

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let size = matrix.len();

    // Transpose (swap rows and cols)
    for i in 0..size {
        for j in 0..i {
            swap((i, j), (j, i), matrix);
        }
    }

    // Reverse each row
    for i in 0..size {
        matrix[i].reverse();
    }
}

fn rotate_cloned(matrix: &mut Vec<Vec<i32>>) {
    let size = matrix.len();
    let mut rotated = matrix.clone();

    for i in 0..size {
        for j in 0..size {
            rotated[j][i] = matrix[size - i - 1][j];
        }
    }

    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = rotated[i][j];
        }
    }
}

fn swap(i: (usize, usize), j: (usize, usize), matrix: &mut Vec<Vec<i32>>) {
    let tmp = matrix[i.0][i.1];
    matrix[i.0][i.1] = matrix[j.0][j.1];
    matrix[j.0][j.1] = tmp;
}

#[cfg(test)]
mod tests {
    use crate::rotate::{rotate, rotate_cloned};
    use rstest::rstest;

    #[rstest(input, expected,
    case(
    & mut vec ! [ vec ! [1, 2, 3], vec ! [4, 5, 6], vec ! [7, 8, 9] ],
    & mut vec ! [ vec ! [7, 4, 1], vec ! [8, 5, 2], vec ! [9, 6, 3] ],
    ),
    case(
    & mut vec ! [ vec ! [5, 1, 9, 11], vec ! [2, 4, 8, 10], vec ! [13, 3, 6, 7], vec ! [15, 14, 12, 16] ],
    & mut vec ! [ vec ! [15, 13, 2, 5], vec ! [14, 3, 4, 1], vec ! [12, 6, 8, 9], vec ! [16, 7, 10, 11] ],
    ),
    )]
    fn test_rotate(input: &mut Vec<Vec<i32>>, expected: &mut Vec<Vec<i32>>) {
        rotate(input);

        assert_eq!(expected, input);
    }

    #[rstest(input, expected,
    case(
    & mut vec ! [ vec ! [1, 2, 3], vec ! [4, 5, 6], vec ! [7, 8, 9] ],
    & mut vec ! [ vec ! [7, 4, 1], vec ! [8, 5, 2], vec ! [9, 6, 3] ],
    ),
    case(
    & mut vec ! [ vec ! [5, 1, 9, 11], vec ! [2, 4, 8, 10], vec ! [13, 3, 6, 7], vec ! [15, 14, 12, 16] ],
    & mut vec ! [ vec ! [15, 13, 2, 5], vec ! [14, 3, 4, 1], vec ! [12, 6, 8, 9], vec ! [16, 7, 10, 11] ],
    ),
    )]
    fn test_rotate_cloned(input: &mut Vec<Vec<i32>>, expected: &mut Vec<Vec<i32>>) {
        rotate_cloned(input);

        assert_eq!(expected, input);
    }
}