use crate::board::{Board, BOARD_SIZE, ChangesWithGuess, Location};

pub fn combo(board: &mut Board) -> Option<Board> {
    board.set_all_poss();
    let mut history_stack: Vec<ChangesWithGuess> = Vec::new();

    while let Some((location, count)) = board.find_fewest_poss_location() {
        let location = location;
        let count = count;
        println!("{}", board.print_board());
        if count != 0 {
            let cell = &mut board.grid[location.row][location.col];
            for i in 0..BOARD_SIZE {
                if cell.poss[i] {
                    let guess = (i + 1) as u8;
                    cell.val = guess;
                    history_stack.push(ChangesWithGuess {
                        guess,
                        guess_location: location,
                        changes: board.update_affected_poss_with_changes(location, guess),
                    });
                    break;
                }
            }
        } else {
            let changes_with_guess = history_stack.pop();
            if changes_with_guess.is_none() {
                return None;
            }

            let ChangesWithGuess {
                guess,
                guess_location: Location { row, col },
                changes,
            } = changes_with_guess.unwrap();
            //reverse possibility of guess for affected cells
            board.reverse_affected_poss(changes, guess);
            //reverse guess, BUT remember it was a bad guess!
            board.grid[row][col].val = 0;
            board.grid[row][col].poss[(guess - 1) as usize] = false;
        }
    }
    Some(*board)
}
