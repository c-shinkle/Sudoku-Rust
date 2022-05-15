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

    pub fn find_blank_cell(&self) -> Option<(usize, usize, &Cell)> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let cell = &self.grid[row][col];
                if cell.is_blank() {
                    return Some((row, col, cell));
                }
            }
        }
        None
    }

    pub fn find_fewest_poss(&self) -> Option<(usize, usize, &Cell)> {
        let mut smallest_count = 10;
        let mut fewest_so_far = None;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let cell = &self.grid[row][col];
                if cell.is_blank() {
                    let count = cell.poss.iter().filter(|p| **p).count();
                    if smallest_count > count {
                        smallest_count = count;
                        fewest_so_far = Some((row, col, cell));
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
            //is this check even necessary?
            if cell.is_blank() {
                cell.poss[(val - 1) as usize] = false;
            }
        }
        //update col
        for i in 0..BOARD_SIZE {
            let mut cell = &mut self.grid[i][col];
            //is this check even necessary?
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
            //is this check even necessary?
            if cell.is_blank() {
                cell.poss[(val - 1) as usize] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use std::default::default;
    use super::*;

    const SIMPLE_POSS: [bool; 9] = [true; BOARD_SIZE];
    //TODO how to fix formatting
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
        given
            .set_board_file("./res/given_valid_board_should_set_poss.txt")
            .expect("file to present");
        //when
        given.set_all_poss();
        //then
        let actual = given.grid[0][0].poss;
        let expected = [false, false, false, false, false, true, false, false, true];
        assert_eq!(expected, actual);
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
        given.grid[4][4] = Cell {
            val: 8,
            poss: false_poss,
        };
        //when
        let mut actual = given.clone();
        actual.update_affected_poss(4, 4, 8);
        //then
        let expected = [
            //row 0
            [
                Cell {
                    val: 4,
                    poss: false_poss,
                },
                Cell {
                    val: 5,
                    poss: false_poss,
                },
                Cell {
                    val: 1,
                    poss: false_poss,
                },
                Cell {
                    val: 3,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, false, false, false, true],
                },
                Cell {
                    val: 2,
                    poss: false_poss,
                },
                Cell {
                    val: 7,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, false, false, true, false],
                },
                Cell {
                    val: 6,
                    poss: false_poss,
                },
            ],
            //row 1
            [
                Cell {
                    val: 9,
                    poss: false_poss,
                },
                Cell {
                    val: 2,
                    poss: false_poss,
                },
                Cell {
                    val: 3,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, false, false, true, false],
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, true, false, false, false],
                },
                Cell {
                    val: 7,
                    poss: false_poss,
                },
                Cell {
                    val: 1,
                    poss: false_poss,
                },
                Cell {
                    val: 4,
                    poss: false_poss,
                },
                Cell {
                    val: 5,
                    poss: false_poss,
                },
            ],
            //row 2
            [
                Cell {
                    val: 8,
                    poss: [false, false, false, false, false, false, false, true, false],
                },
                Cell {
                    val: 6,
                    poss: false_poss,
                },
                Cell {
                    val: 7,
                    poss: false_poss,
                },
                Cell {
                    val: 1,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, true, false, false, false, false],
                },
                Cell {
                    val: 4,
                    poss: false_poss,
                },
                Cell {
                    val: 3,
                    poss: false_poss,
                },
                Cell {
                    val: 2,
                    poss: false_poss,
                },
                Cell {
                    val: 9,
                    poss: false_poss,
                },
            ],
            //row 3
            [
                Cell {
                    val: 1,
                    poss: false_poss,
                },
                Cell {
                    val: 3,
                    poss: false_poss,
                },
                Cell {
                    val: 6,
                    poss: false_poss,
                },
                Cell {
                    val: 5,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, true, false, false, false, false, false],
                },
                Cell {
                    val: 9,
                    poss: false_poss,
                },
                Cell {
                    val: 2,
                    poss: false_poss,
                },
                Cell {
                    val: 7,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, false, false, true, false],
                },
            ],
            //row 4
            [
                Cell {
                    val: 0,
                    poss: [false, true, false, false, false, false, false, false, false],
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, true, false, false, false, false, false],
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, true, false, false, false, false],
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, false, true, false, false],
                },
                Cell {
                    val: 8,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, true, false, false, false, false, false, false],
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, true, false, false, false],
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, false, false, false, true],
                },
                Cell {
                    val: 0,
                    poss: [true, false, false, false, false, false, false, false, false],
                },
            ],
            //row 5
            [
                Cell {
                    val: 7,
                    poss: false_poss,
                },
                Cell {
                    val: 9,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, false, false, true, false],
                },
                Cell {
                    val: 6,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, true, false, false, false, false, false, false, false],
                },
                Cell {
                    val: 1,
                    poss: false_poss,
                },
                Cell {
                    val: 4,
                    poss: false_poss,
                },
                Cell {
                    val: 5,
                    poss: false_poss,
                },
                Cell {
                    val: 3,
                    poss: false_poss,
                },
            ],
            //row 6
            [
                Cell {
                    val: 5,
                    poss: false_poss,
                },
                Cell {
                    val: 8,
                    poss: [false, false, false, false, false, false, false, true, false],
                },
                Cell {
                    val: 2,
                    poss: false_poss,
                },
                Cell {
                    val: 4,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, true, false, false, false, false, false, false],
                },
                Cell {
                    val: 6,
                    poss: false_poss,
                },
                Cell {
                    val: 9,
                    poss: false_poss,
                },
                Cell {
                    val: 1,
                    poss: false_poss,
                },
                Cell {
                    val: 7,
                    poss: false_poss,
                },
            ],
            //row 7
            [
                Cell {
                    val: 6,
                    poss: false_poss,
                },
                Cell {
                    val: 1,
                    poss: false_poss,
                },
                Cell {
                    val: 9,
                    poss: false_poss,
                },
                Cell {
                    val: 2,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, false, true, false, false],
                },
                Cell {
                    val: 5,
                    poss: false_poss,
                },
                Cell {
                    val: 8,
                    poss: [false, false, false, false, false, false, false, true, false],
                },
                Cell {
                    val: 3,
                    poss: false_poss,
                },
                Cell {
                    val: 4,
                    poss: false_poss,
                },
            ],
            //row 8
            [
                Cell {
                    val: 3,
                    poss: false_poss,
                },
                Cell {
                    val: 7,
                    poss: false_poss,
                },
                Cell {
                    val: 4,
                    poss: false_poss,
                },
                Cell {
                    val: 9,
                    poss: false_poss,
                },
                Cell {
                    val: 0,
                    poss: [true, false, false, false, false, false, false, false, false],
                },
                Cell {
                    val: 0,
                    poss: [false, false, false, false, false, false, false, true, false],
                },
                Cell {
                    val: 5,
                    poss: false_poss,
                },
                Cell {
                    val: 6,
                    poss: false_poss,
                },
                Cell {
                    val: 2,
                    poss: false_poss,
                },
            ],
        ];
        //assert rows match
        let expected_middle_row_poss: Vec<[bool; BOARD_SIZE]> =
            expected[4].iter().map(|cell| cell.poss).collect();
        let actual_middle_row_poss: Vec<[bool; BOARD_SIZE]> =
            actual.grid[4].iter().map(|cell| cell.poss).collect();
        for i in 0..BOARD_SIZE {
            assert_eq!(
                actual_middle_row_poss[i], expected_middle_row_poss[i],
                "row {}",
                i
            );
        }
        // assert cols match
        let expected_middle_col_poss: Vec<[bool; BOARD_SIZE]> =
            expected.iter().map(|row| row[4].poss).collect();
        let actual_middle_col_poss: Vec<[bool; BOARD_SIZE]> =
            actual.grid.iter().map(|row| row[4].poss).collect();
        for i in 0..BOARD_SIZE {
            assert_eq!(
                actual_middle_col_poss[i], expected_middle_col_poss[i],
                "col {}",
                i
            );
        }
        //assert box match
        let mut expected_middle_box_cell: Vec<Cell> = Vec::new();
        expected_middle_box_cell.extend(expected[3..6].iter().map(|row| row[3]));
        expected_middle_box_cell.extend(expected[3..6].iter().map(|row| row[4]));
        expected_middle_box_cell.extend(expected[3..6].iter().map(|row| row[5]));
        let mut actual_middle_box_poss: Vec<Cell> = Vec::new();
        actual_middle_box_poss.extend(actual.grid[3..6].iter().map(|row| row[3]));
        actual_middle_box_poss.extend(actual.grid[3..6].iter().map(|row| row[4]));
        actual_middle_box_poss.extend(actual.grid[3..6].iter().map(|row| row[5]));
        for i in 0..BOARD_SIZE {
            assert_eq!(
                actual_middle_box_poss[i], expected_middle_box_cell[i],
                "box {}",
                i
            );
        }
    }
}
