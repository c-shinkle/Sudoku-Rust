use crate::board::{Board, BOARD_SIZE};

pub fn fewest_poss(board: &mut Board) -> Option<Board> {
    board.set_all_poss();
    let maybe_cell = board.find_fewest_poss();
    if let Some((row, col, cell)) = maybe_cell {
        for i in 0..BOARD_SIZE {
            if cell.poss[i] {
                let mut copied_board = *board;
                copied_board.grid[row][col].val = (i + 1) as u8;
                let maybe_solved = fewest_poss(&mut copied_board);
                if maybe_solved.is_some() {
                    return maybe_solved;
                }
            }
        }
        return None;
    }
    Some(*board)
}