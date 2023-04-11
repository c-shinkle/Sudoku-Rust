use crate::board::{Board, BOARD_SIZE};

pub fn fewest_poss(board: &mut Board) -> bool {
    if let Some(solved_board) = helper(board) {
        *board = solved_board;
        return true;
    }
    false
}

fn helper(board: &mut Board) -> Option<Board> {
    board.set_all_poss();
    if let Some((row, col)) = board.find_fewest_poss() {
        for i in 0..BOARD_SIZE {
            if board.grid[row][col].poss[i] {
                let mut copied_board = *board;
                copied_board.grid[row][col].val = (i + 1) as u8;
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
