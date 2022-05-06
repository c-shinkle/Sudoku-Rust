use crate::board::{Board, BOARD_SIZE};

pub fn naive(board: &mut Board) -> Option<Board> {
    populate_possible_values(board);
    //then recursively guess and check all blank cells
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let cell = board.grid[row][col];
            if cell.is_blank() {
                //check each possibility until one works
                for i in 1..=BOARD_SIZE {
                    if cell.poss[i - 1] {
                        let mut copied_board = *board;
                        copied_board.grid[row][col].val = i as u8;
                        let maybe_solved = naive(&mut copied_board);
                        //Almost every call will return true
                        if maybe_solved.is_some() {
                            return maybe_solved;
                        }
                    }
                }
                return None;
            }
        }
    }
    Some(*board)
}

pub fn populate_possible_values(board: &mut Board) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            //check row
            for i in 0..BOARD_SIZE {
                let val = board.grid[row][i].val;
                if val != 0 {
                    let val_index = (val - 1) as usize;
                    board.grid[row][col].poss[val_index] = false;
                }
            }
            //check col
            for i in 0..BOARD_SIZE {
                let val = board.grid[i][col].val;
                if val != 0 {
                    let val_index = (val - 1) as usize;
                    board.grid[row][col].poss[val_index] = false;
                }
            }
            //check box
            let box_row = row / 3;
            let box_col = col / 3;
            for i in 0..BOARD_SIZE {
                let grid_row = box_row * 3 + (i / 3);
                let grid_col = box_col * 3 + (i % 3);
                let val = board.grid[grid_row][grid_col].val;
                if val != 0 {
                    let val_index = (val - 1) as usize;
                    board.grid[row][col].poss[val_index] = false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn given_valid_board_should_solve_board() {
        //given
        let mut given = Board::new();
        given.set_board_string(
            "\
            003174258\
            178325649\
            254689731\
            821437596\
            496852317\
            735961824\
            589713462\
            317246985\
            042598173\
        ",
        );
        //when
        let actual = naive(&mut given);
        //then
        assert!(actual.is_some());
        let mut expected = Board::new();
        expected.set_board_string(
            "\
            963174258\
            178325649\
            254689731\
            821437596\
            496852317\
            735961824\
            589713462\
            317246985\
            642598173\
        ",
        );
        assert_eq!(expected.print_board(), actual.unwrap().print_board());
    }

    #[test]
    fn given_valid_board_should_set_poss() {
        let mut given = Board::new();
        given.set_board_string(
            "\
            003174258\
            178325649\
            254689731\
            821437596\
            496852317\
            735961824\
            589713462\
            317246985\
            042598173\
        ",
        );
        let expected_poss = [false, false, false, false, false, true, false, false, true];

        populate_possible_values(&mut given);

        let actual_poss = given.grid[0][0].poss;
        assert_eq!(expected_poss, actual_poss);
    }
}
