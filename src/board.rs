pub const BOARD_SIZE: usize = 9;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub val: u8,
    pub poss: [bool; BOARD_SIZE],
}

#[derive(Clone, Debug)]
pub struct Board {
    pub grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

const TOO_FEW_CHARS: &str = "Should be exactly 81 chars in string slice.";

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[Cell {
                val: 0,
                poss: [true; BOARD_SIZE],
            }; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn set_board_string(&mut self, values: &str) {
        let mut chars = values.chars();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                self.grid[row][col].val = chars.next().expect(TOO_FEW_CHARS) as u8 - b'0';
            }
        }
    }

    pub fn print_board(&self) -> String {
        let mut chars = String::with_capacity(132);

        Board::add_ith_row(self.grid[0], &mut chars);
        Board::add_ith_row(self.grid[1], &mut chars);
        Board::add_ith_row(self.grid[2], &mut chars);

        chars.push_str("--- --- ---\n");

        Board::add_ith_row(self.grid[3], &mut chars);
        Board::add_ith_row(self.grid[4], &mut chars);
        Board::add_ith_row(self.grid[5], &mut chars);

        chars.push_str("--- --- ---\n");

        Board::add_ith_row(self.grid[6], &mut chars);
        Board::add_ith_row(self.grid[7], &mut chars);
        Board::add_ith_row(self.grid[8], &mut chars);

        chars
    }

    fn add_ith_row(row: [Cell; 9], chars: &mut String) {
        chars.push((row[0].val + 48) as char);
        chars.push((row[1].val + 48) as char);
        chars.push((row[2].val + 48) as char);

        chars.push('|');

        chars.push((row[3].val + 48) as char);
        chars.push((row[4].val + 48) as char);
        chars.push((row[5].val + 48) as char);

        chars.push('|');

        chars.push((row[6].val + 48) as char);
        chars.push((row[7].val + 48) as char);
        chars.push((row[8].val + 48) as char);

        chars.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_ref_to_blank_board_should_print_board() {
        //given
        let board = Board::new();
        //when
        let actual = board.print_board();
        //then
        assert_eq!(String::from(
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
            000|000|000\n\
        ",
        ), actual);
    }

    #[test]
    fn given_string_should_set_board() {
        //given
        let given = "\
            123456789\
            123456789\
            123456789\
            123456789\
            123456789\
            123456789\
            123456789\
            123456789\
            123456789\
        ";
        //when
        let mut actual = Board::new();
        actual.set_board_string(&given);
        //then
        let poss = [true; BOARD_SIZE];
        assert_eq!([[
            Cell { val: 1, poss },
            Cell { val: 2, poss },
            Cell { val: 3, poss },
            Cell { val: 4, poss },
            Cell { val: 5, poss },
            Cell { val: 6, poss },
            Cell { val: 7, poss },
            Cell { val: 8, poss },
            Cell { val: 9, poss },
        ]; BOARD_SIZE], actual.grid);
    }

    #[test]
    #[should_panic(expected = "Should be exactly 81 chars in string slice.")]
    fn given_string_too_short_should_panic_with_message() {
        //given
        let given = "\
            123456789\
            123456789\
            123456789\
            123456789\
            123456789\
            123456789\
            123456789\
            123456789\
        ";
        //when
        Board::new().set_board_string(&given);
        //then
    }
}
