/// Determine if a 9 x 9 Sudoku board is valid.
/// Only the filled cells need to be validated according to the following rules:
///
/// Each row must contain the digits 1-9 without repetition.
/// Each column must contain the digits 1-9 without repetition.
/// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
///
/// Note:
/// A Sudoku board (partially filled) could be valid but is not necessarily solvable.
/// Only the filled cells need to be validated according to the mentioned rules.
///
/// Input: board =
/// [["5","3",".", ".","7",".", ".",".","."]
/// ,["6",".",".", "1","9","5", ".",".","."]
/// ,[".","9","8", ".",".",".", ".","6","."]
///
/// ,["8",".",".", ".","6",".", ".",".","3"]
/// ,["4",".",".", "8",".","3", ".",".","1"]
/// ,["7",".",".", ".","2",".", ".",".","6"]
///
/// ,[".","6",".", ".",".",".", "2","8","."]
/// ,[".",".",".", "4","1","9", ".",".","5"]
/// ,[".",".",".", ".","8",".", ".","7","9"]]
/// Output: true

const EMPTY_CELL: char = '.';
const BOX_SIZE: usize = 3;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashMap;

    let mut box_registry: HashMap<usize, Vec<char>> = HashMap::new();

    for row_index in 0..board.len() {
        let mut row_registry: Vec<char> = Vec::new();
        let mut col_registry: Vec<char> = Vec::new();

        for col_index in 0..board[row_index].len() {
            let row_value = board[row_index][col_index];
            let col_value = board[col_index][row_index];

            // Validate row
            if row_registry.contains(&row_value) && row_value != EMPTY_CELL { return false; }
            row_registry.push(row_value);

            // Validate cell
            if col_registry.contains(&col_value) && col_value != EMPTY_CELL { return false; }
            col_registry.push(col_value);

            // Validate box
            let box_index = row_index / BOX_SIZE * BOX_SIZE + col_index / BOX_SIZE;

            if let Some(box_elements) = box_registry.get_mut(&box_index) {
                if box_elements.contains(&row_value) && row_value != EMPTY_CELL {
                    return false;
                }
                box_elements.push(row_value);
            } else {
                box_registry.insert(box_index, vec![row_value]);
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_valid_sudoku;
    use rstest::rstest;

    #[rstest(board, expected, case(
    vec ! [
    vec ! ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
    vec ! ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
    vec ! ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
    vec ! ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
    vec ! ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
    vec ! ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
    vec ! ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
    vec ! ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
    vec ! ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ],
    true
    ))]
    fn test_is_valid_sudoku(board: Vec<Vec<char>>, expected: bool) {
        assert_eq!(expected, is_valid_sudoku(board));
    }
}