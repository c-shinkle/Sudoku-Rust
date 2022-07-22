use crate::board::{Board, BOARD_SIZE};

pub fn naive(board: &mut Board) -> Option<Board> {
    board.set_all_poss();
    let maybe_cell = board.find_blank_cell();
    if let Some((row, col, cell)) = maybe_cell {
        for i in 0..BOARD_SIZE {
            if cell.poss[i] {
                let mut copied_board = *board;
                copied_board.grid[row][col].val = (i + 1) as u8;
                let maybe_solved = naive(&mut copied_board);
                if maybe_solved.is_some() {
                    return maybe_solved;
                }
            }
        }
        return None;
    }
    Some(*board)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
