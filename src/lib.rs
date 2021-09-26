pub const BOARD_SIZE: usize = 9;

pub struct Board {
    grid: [ [u8;  BOARD_SIZE]; BOARD_SIZE]
}

impl Board {
    pub fn new() -> Board {
        Board { grid: [[0; BOARD_SIZE]; BOARD_SIZE] }
    }
}

fn add_ith_row(board: &Board, chars: &mut Vec<char>, grid_row: usize) {
    let row = board.grid[grid_row];
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

fn add_blank_row(chars: &mut Vec<char>) {
    for c in "--- --- ---\n".chars() {
        chars.push(c);
    }
}

pub fn print_board(board: &Board) -> String {
    let mut chars = Vec::new();
    
    add_ith_row(board, &mut chars, 0);
    add_ith_row(board, &mut chars, 1);
    add_ith_row(board, &mut chars, 2);

    add_blank_row(&mut chars);

    add_ith_row(board, &mut chars, 3);
    add_ith_row(board, &mut chars, 4);
    add_ith_row(board, &mut chars, 5);

    add_blank_row(&mut chars);

    add_ith_row(board, &mut chars, 6);
    add_ith_row(board, &mut chars, 7);
    add_ith_row(board, &mut chars, 8);

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_ref_to_blank_board_should_print_board() {
        let expected = String::from(
            "000|000|000\n\
            000|000|000\n\
            000|000|000\n\
            --- --- ---\n\
            000|000|000\n\
            000|000|000\n\
            000|000|000\n\
            --- --- ---\n\
            000|000|000\n\
            000|000|000\n\
            000|000|000\n"
        );
        let board: Board = Board::new();

        let actual = print_board(&board);
        
        assert_eq!(actual, expected);
    }
}