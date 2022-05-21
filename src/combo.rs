use crate::board::{Board, BOARD_SIZE};

pub fn combo(board: &mut Board) -> Option<Board> {
    board.set_all_poss();
    helper(board)
}

fn helper(board: &mut Board) -> Option<Board> {
    let maybe_cell = board.find_fewest_poss();
    if let Some((row, col, cell)) = maybe_cell {
        for i in 0..BOARD_SIZE {
            if cell.poss[i] {
                let mut copied_board = *board;
                let guess = i as u8 + 1;
                copied_board.grid[row][col].val = guess;
                copied_board.update_affected_poss(row, col, guess);
                let maybe_solved = helper(&mut copied_board);
                if maybe_solved.is_some() {
                    return maybe_solved;
                }
            }
        }
        None
    } else {
        Some(*board)
    }
}
