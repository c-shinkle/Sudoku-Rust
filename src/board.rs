const BOARD_SIZE: usize = 9;

pub struct Board {
    grid: [[u8; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[0; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

pub fn print_board(board: &Board) -> String {
    let mut chars = String::with_capacity(132);

    add_ith_row(board.grid[0], &mut chars);
    add_ith_row(board.grid[1], &mut chars);
    add_ith_row(board.grid[2], &mut chars);

    chars.push_str("--- --- ---\n");

    add_ith_row(board.grid[3], &mut chars);
    add_ith_row(board.grid[4], &mut chars);
    add_ith_row(board.grid[5], &mut chars);

    chars.push_str("--- --- ---\n");

    add_ith_row(board.grid[6], &mut chars);
    add_ith_row(board.grid[7], &mut chars);
    add_ith_row(board.grid[8], &mut chars);

    chars
}

fn add_ith_row(row: [u8; 9], chars: &mut String) {
    chars.push((row[0] + 48) as char);
    chars.push((row[1] + 48) as char);
    chars.push((row[2] + 48) as char);

    chars.push('|');

    chars.push((row[3] + 48) as char);
    chars.push((row[4] + 48) as char);
    chars.push((row[5] + 48) as char);

    chars.push('|');

    chars.push((row[6] + 48) as char);
    chars.push((row[7] + 48) as char);
    chars.push((row[8] + 48) as char);

    chars.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_ref_to_blank_board_should_print_board() {
        let expected = String::from(
            "\
            000|000|000\n\
            000|000|000\n\
            000|000|000\n\
            --- --- ---\n\
            000|000|000\n\
            000|000|000\n\
            000|000|000\n\
            --- --- ---\n\
            000|000|000\n\
            000|000|000\n\
            000|000|000\n",
        );
        let board = Board::new();

        let actual = print_board(&board);

        assert_eq!(actual, expected);
    }
}
