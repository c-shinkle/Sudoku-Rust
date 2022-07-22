use crate::board::{Board, BOARD_SIZE, ChangesWithGuess};

pub fn combo(board: &mut Board) -> Option<Board> {
    board.set_all_poss();
    let mut history: Vec<ChangesWithGuess> = Vec::new();

    while let Some((row, col, cell)) = board.find_fewest_poss() {
        let mut did_not_find_guess = true;

        for i in 0..BOARD_SIZE {
            if cell.poss[i] {
                did_not_find_guess = false;
                let guess = (i + 1) as u8;
                board.grid[row][col].val = guess;
                board.grid[row][col].poss[i] = false;
                history.push(board.update_affected_poss_with_changes(row, col, guess));
                break;
            }
        }

        if did_not_find_guess {
            let ChangesWithGuess { changes, guess } =
                history.pop().expect("This board is unsolvable!");
            board.reverse_affected_poss(changes);
            board.grid[row][col].poss[(guess - 1) as usize] = false;
        }
    }
    Some(*board)
}
