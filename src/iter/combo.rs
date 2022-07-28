use crate::board::{Board, History, BOARD_SIZE};

pub fn combo(board: &mut Board) -> Option<Board> {
    board.set_all_poss();
    let mut history_stack: Vec<History> = Vec::new();

    while let Some((row, col, count)) = board.find_fewest_poss_count() {
        if count == 0 {
            let maybe_board = history_stack.pop();
            maybe_board.as_ref()?;
            let History {
                board: prev_board,
                guess,
                row: prev_row,
                col: prev_col,
            } = maybe_board.unwrap();
            *board = prev_board;
            board.grid[prev_row][prev_col].poss[(guess - 1) as usize] = false;
        } else {
            for i in 0..BOARD_SIZE {
                if board.grid[row][col].poss[i] {
                    let guess = (i + 1) as u8;
                    history_stack.push(History {
                        board: *board,
                        guess,
                        row,
                        col,
                    });
                    board.grid[row][col].val = guess;
                    board.update_affected_poss(row, col, guess);
                    break;
                }
            }
        }
    }
    Some(*board)
}
