use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub const BOARD_SIZE: usize = 9;
const TOO_FEW_CHARS: &str = "Should be exactly 81 chars in string slice.";

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub val: u8,
    pub poss: [bool; BOARD_SIZE],
}

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Cell {
    #[inline]
    pub fn is_blank(&self) -> bool {
        self.val == 0
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

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

    pub fn set_board_file(&mut self, path: &str) -> Result<()> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::with_capacity(BOARD_SIZE + 1);
        let mut values = String::with_capacity(BOARD_SIZE * BOARD_SIZE);
        for _ in 0..BOARD_SIZE {
            reader.read_line(&mut buffer)?;
            values.push_str(&buffer[0..BOARD_SIZE]);
            buffer.clear();
        }
        self.set_board_string(values.as_str());
        Ok(())
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

    pub fn set_all_poss(&mut self) {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                //check row
                for i in 0..BOARD_SIZE {
                    let val = self.grid[row][i].val;
                    if val != 0 {
                        let val_index = (val - 1) as usize;
                        self.grid[row][col].poss[val_index] = false;
                    }
                }
                //check col
                for i in 0..BOARD_SIZE {
                    let val = self.grid[i][col].val;
                    if val != 0 {
                        let val_index = (val - 1) as usize;
                        self.grid[row][col].poss[val_index] = false;
                    }
                }
                //check box
                let box_row = row / 3;
                let box_col = col / 3;
                for i in 0..BOARD_SIZE {
                    let grid_row = box_row * 3 + (i / 3);
                    let grid_col = box_col * 3 + (i % 3);
                    let val = self.grid[grid_row][grid_col].val;
                    if val != 0 {
                        self.grid[row][col].poss[(val - 1) as usize] = false;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_POSS: [bool; 9] = [true; BOARD_SIZE];
    const SIMPLE_BOARD: [[Cell; 9]; 9] = [[
        Cell {
            val: 1,
            poss: SIMPLE_POSS,
        },
        Cell {
            val: 2,
            poss: SIMPLE_POSS,
        },
        Cell {
            val: 3,
            poss: SIMPLE_POSS,
        },
        Cell {
            val: 4,
            poss: SIMPLE_POSS,
        },
        Cell {
            val: 5,
            poss: SIMPLE_POSS,
        },
        Cell {
            val: 6,
            poss: SIMPLE_POSS,
        },
        Cell {
            val: 7,
            poss: SIMPLE_POSS,
        },
        Cell {
            val: 8,
            poss: SIMPLE_POSS,
        },
        Cell {
            val: 9,
            poss: SIMPLE_POSS,
        },
    ]; BOARD_SIZE];

    #[test]
    fn given_ref_to_blank_board_should_print_board() {
        //given
        let given = Board::new();
        //when
        let actual = given.print_board();
        //then
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
        assert_eq!(expected, actual);
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
        //then;
        assert_eq!(SIMPLE_BOARD, actual.grid);
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
        Board::new().set_board_string(given);
        //then
    }

    #[test]
    fn given_path_should_set_board() {
        //given
        let path = "./res/given_path_should_set_board.txt";
        let mut actual = Board::new();
        //when
        let result = actual.set_board_file(path);
        //then
        assert!(result.is_ok());
        assert_eq!(SIMPLE_BOARD, actual.grid);
    }

    #[test]
    fn given_bad_path_should_return_error() {
        //given
        //when
        let actual = Board::new().set_board_file("./bad/path");
        //then
        assert!(actual.is_err());
    }

    #[test]
    fn given_valid_board_should_set_poss() {
        //given
        let mut given = Board::new();
        given.set_board_file("./res/given_valid_board_should_set_poss.txt");
        //when
        given.set_all_poss();
        //then
        let actual = given.grid[0][0].poss;
        let expected = [false, false, false, false, false, true, false, false, true];
        assert_eq!(expected, actual);
    }
}
