use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub const BOARD_SIZE: usize = 9;
const TRUE_POSS: [bool; BOARD_SIZE] = [true; BOARD_SIZE];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub val: u8,
    pub poss: [bool; BOARD_SIZE],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board {
    pub grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct History {
    pub board: Board,
    pub guess: u8,
    pub row: usize,
    pub col: usize,
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

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_board())
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[Cell {
                val: 0,
                poss: TRUE_POSS,
            }; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn set_board_string(&mut self, values: &str) {
        let mut chars = values.chars();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let msg = "Should be exactly 81 chars in string slice.";
                self.grid[row][col].val = chars.next().expect(msg) as u8 - b'0';
            }
        }
    }

    pub fn set_board_file(&mut self, path: &str) -> io::Result<()> {
        let mut board_string = String::new();
        for line in BufReader::new(File::open(path)?).lines() {
            board_string.push_str(&line?);
        }
        self.set_board_string(&board_string);
        Ok(())
    }

    pub fn print_board(&self) -> String {
        // 12 chars * 11 lines
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

    fn add_ith_row(row: [Cell; BOARD_SIZE], chars: &mut String) {
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
                    let cell = self.grid[row][i];
                    if !cell.is_blank() {
                        self.grid[row][col].poss[(cell.val - 1) as usize] = false;
                    }
                }
                //check col
                for i in 0..BOARD_SIZE {
                    let cell = self.grid[i][col];
                    if !cell.is_blank() {
                        self.grid[row][col].poss[(cell.val - 1) as usize] = false;
                    }
                }
                //check box
                let box_row = row / 3;
                let box_col = col / 3;
                for i in 0..BOARD_SIZE {
                    let grid_row = box_row * 3 + (i / 3);
                    let grid_col = box_col * 3 + (i % 3);
                    let cell = self.grid[grid_row][grid_col];
                    if !cell.is_blank() {
                        self.grid[row][col].poss[(cell.val - 1) as usize] = false;
                    }
                }
            }
        }
    }

    pub fn find_blank_cell(&self) -> Option<(usize, usize)> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.grid[row][col].is_blank() {
                    return Some((row, col));
                }
            }
        }
        None
    }

    pub fn find_fewest_poss(&self) -> Option<(usize, usize)> {
        let mut smallest_count = 10;
        let mut fewest_so_far = None;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let cell = &self.grid[row][col];
                if cell.is_blank() {
                    let count = cell.poss.iter().filter(|p| **p).count();
                    if smallest_count > count {
                        smallest_count = count;
                        fewest_so_far = Some((row, col));
                    }
                }
            }
        }
        fewest_so_far
    }

    pub fn find_fewest_poss_count(&self) -> Option<(usize, usize, usize)> {
        let mut smallest_count = 10;
        let mut fewest_so_far = None;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let cell = &self.grid[row][col];
                if cell.is_blank() {
                    let count = cell.poss.into_iter().filter(|p| *p).count();
                    if count == 0 {
                        return Some((row, col, 0));
                    } else if smallest_count > count {
                        smallest_count = count;
                        fewest_so_far = Some((row, col, count));
                    }
                }
            }
        }
        fewest_so_far
    }

    pub fn update_affected_poss(&mut self, row: usize, col: usize, val: u8) {
        //update row
        for i in 0..BOARD_SIZE {
            let mut cell = &mut self.grid[row][i];
            if cell.is_blank() {
                cell.poss[(val - 1) as usize] = false;
            }
        }
        //update col
        for i in 0..BOARD_SIZE {
            let mut cell = &mut self.grid[i][col];
            if cell.is_blank() {
                cell.poss[(val - 1) as usize] = false;
            }
        }
        //update box
        let box_row = row / 3;
        let box_col = col / 3;
        for i in 0..BOARD_SIZE {
            let grid_row = box_row * 3 + (i / 3);
            let grid_col = box_col * 3 + (i % 3);
            let mut cell = &mut self.grid[grid_row][grid_col];
            if cell.is_blank() {
                cell.poss[(val - 1) as usize] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    const fn new(val: u8, poss: [bool; BOARD_SIZE]) -> Cell {
        Cell { val, poss }
    }

    const SIMPLE_BOARD: Board = Board {
        grid: [[
            new(1, TRUE_POSS),
            new(2, TRUE_POSS),
            new(3, TRUE_POSS),
            new(4, TRUE_POSS),
            new(5, TRUE_POSS),
            new(6, TRUE_POSS),
            new(7, TRUE_POSS),
            new(8, TRUE_POSS),
            new(9, TRUE_POSS),
        ]; BOARD_SIZE],
    };

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
        assert_eq!(actual, SIMPLE_BOARD);
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
        assert_eq!(actual, SIMPLE_BOARD);
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
        given
            .set_board_file("./res/given_valid_board_should_set_poss.txt")
            .expect("File to be present");
        //when
        given.set_all_poss();
        let actual = given.grid[0][0].poss;
        //then
        assert_eq!(
            actual,
            [false, false, false, false, false, true, false, false, true]
        );
    }

    #[test]
    fn given_new_val_should_update_affected_cells() {
        //given
        let mut given = Board::new();
        given
            .set_board_file("./res/given_new_val_should_update_affected_cells.txt")
            .expect("File to be present");
        given.set_all_poss();
        let false_poss = [false; BOARD_SIZE];
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let mut cell = &mut given.grid[row][col];
                if !cell.is_blank() {
                    cell.poss = false_poss;
                }
            }
        }
        given.grid[4][4].val = 8;
        given.grid[4][4].poss = false_poss;
        //when
        given.update_affected_poss(4, 4, 8);
        let actual = given;
        //then
        #[rustfmt::skip]
            let grid = [
            //row 0
            [
                new(4, false_poss),
                new(5, false_poss),
                new(1, false_poss),
                new(3, false_poss),
                new(0, [false, false, false, false, false, false, false, false, true]),
                new(2, false_poss),
                new(7, false_poss),
                new(0, [false, false, false, false, false, false, false, true, false]),
                new(6, false_poss),
            ],
            //row 1
            [
                new(9, false_poss),
                new(2, false_poss),
                new(3, false_poss),
                new(0, [false, false, false, false, false, false, false, true, false]),
                new(0, [false, false, false, false, false, true, false, false, false]),
                new(7, false_poss),
                new(1, false_poss),
                new(4, false_poss),
                new(5, false_poss),
            ],
            //row 2
            [
                new(0, [false, false, false, false, false, false, false, true, false]),
                new(6, false_poss),
                new(7, false_poss),
                new(1, false_poss),
                new(0, [false, false, false, false, true, false, false, false, false]),
                new(4, false_poss),
                new(3, false_poss),
                new(2, false_poss),
                new(9, false_poss),
            ],
            //row 3
            [
                new(1, false_poss),
                new(3, false_poss),
                new(6, false_poss),
                new(5, false_poss),
                new(0, [false, false, false, true, false, false, false, false, false]),
                new(9, false_poss),
                new(2, false_poss),
                new(7, false_poss),
                new(0, [false, false, false, false, false, false, false, true, false]),
            ],
            //row 4
            [
                new(0, [false, true, false, false, false, false, false, false, false]),
                new(0, [false, false, false, true, false, false, false, false, false]),
                new(0, [false, false, false, false, true, false, false, false, false]),
                new(0, [false, false, false, false, false, false, true, false, false]),
                new(8, false_poss),
                new(0, [false, false, true, false, false, false, false, false, false]),
                new(0, [false, false, false, false, false, true, false, false, false]),
                new(0, [false, false, false, false, false, false, false, false, true]),
                new(0, [true, false, false, false, false, false, false, false, false]),
            ],
            //row 5
            [
                new(7, false_poss),
                new(9, false_poss),
                new(0, [false, false, false, false, false, false, false, true, false]),
                new(6, false_poss),
                new(0, [false, true, false, false, false, false, false, false, false]),
                new(1, false_poss),
                new(4, false_poss),
                new(5, false_poss),
                new(3, false_poss),
            ],
            //row 6
            [
                new(5, false_poss),
                new(0, [false, false, false, false, false, false, false, true, false]),
                new(2, false_poss),
                new(4, false_poss),
                new(0, [false, false, true, false, false, false, false, false, false]),
                new(6, false_poss),
                new(9, false_poss),
                new(1, false_poss),
                new(7, false_poss),
            ],
            //row 7
            [
                new(6, false_poss),
                new(1, false_poss),
                new(9, false_poss),
                new(2, false_poss),
                new(0, [false, false, false, false, false, false, true, false, false]),
                new(5, false_poss),
                new(0, [false, false, false, false, false, false, false, true, false]),
                new(3, false_poss),
                new(4, false_poss),
            ],
            //row 8
            [
                new(3, false_poss),
                new(7, false_poss),
                new(4, false_poss),
                new(9, false_poss),
                new(0, [true, false, false, false, false, false, false, false, false]),
                new(0, [false, false, false, false, false, false, false, true, false]),
                new(5, false_poss),
                new(6, false_poss),
                new(2, false_poss),
            ],
        ];
        // for row in 0..BOARD_SIZE {
        //     for col in 0..BOARD_SIZE {
        //         assert_eq!(
        //             actual[row][col], expected[row][col],
        //             "row {}, col {}",
        //             row, col
        //         );
        //     }
        // }
        assert_eq!(actual, Board { grid });
    }
}
