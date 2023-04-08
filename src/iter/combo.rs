use crate::board::{Board, History};

pub fn combo(board: &mut Board) -> bool {
    board.set_all_poss();
    let mut history_stack: Vec<History> = Vec::new();

    while let Some((row, col, count)) = board.find_fewest_poss_count() {
        if count == 0 {
            let maybe_history = history_stack.pop();
            if maybe_history.is_none() {
                return false;
            }
            let history = maybe_history.unwrap();
            let History {
                board: prev_board,
                guess,
                row: prev_row,
                col: prev_col,
            } = history;
            *board = prev_board;
            board.grid[prev_row][prev_col].poss[(guess - 1) as usize] = false;
        } else if let Some(i) = board.grid[row][col].poss.into_iter().position(|p| p) {
            let guess = (i + 1) as u8;
            history_stack.push(History {
                board: *board,
                guess,
                row,
                col,
            });
            board.grid[row][col].val = guess;
            board.update_affected_poss(row, col, guess);
        }
    }
    true
}
