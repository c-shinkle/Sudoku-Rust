use crate::board::{Board, BOARD_SIZE};

pub fn prev_poss(board: &mut Board) -> bool {
    board.set_all_poss();
    helper(board).is_some()
}

fn helper(board: &mut Board) -> Option<Board> {
    if let Some((row, col)) = board.find_blank_cell() {
        for i in 0..BOARD_SIZE {
            if board.grid[row][col].poss[i] {
                let mut copied_board = *board;
                let guess = (i + 1) as u8;
                copied_board.grid[row][col].val = guess;
                copied_board.update_affected_poss(row, col, guess);
                let maybe_solved = helper(&mut copied_board);
                if maybe_solved.is_some() {
                    return maybe_solved;
                }
            }
        }
        return None;
    }
    Some(*board)
}
