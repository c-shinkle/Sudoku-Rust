use crate::board::{Board, History, BOARD_SIZE};

pub fn combo(board: &mut Board) -> Option<Board> {
    board.set_all_poss();
    let mut history_stack: Vec<History> = Vec::new();

    while let Some((row, col, cell)) = board.find_fewest_poss() {
        // let row = row;
        // let col = col;
        // let cell = cell;
        // println!("{}", board.print_board());
        let mut did_not_find_guess = true;
        for i in 0..BOARD_SIZE {
            if cell.poss[i] {
                did_not_find_guess = false;
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

        if did_not_find_guess {
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
        }
    }
    Some(*board)
}
