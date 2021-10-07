pub mod board_mod {

    pub const BOARD_SIZE: usize = 9;

    #[derive(Copy, Clone, Debug)]
    pub struct Cell {
        pub val: u8,
        pub poss: [bool; BOARD_SIZE],
    }

    impl PartialEq for Cell {
        fn eq(&self, other: &Cell) -> bool {
            self.val == other.val && self.poss == other.poss
        }
    }

    pub struct Board {
        pub grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    }

    impl Board {
        pub fn new() -> Board {
            Board {
                grid: [[Cell {
                    val: 0,
                    poss: [false; BOARD_SIZE],
                }; BOARD_SIZE]; BOARD_SIZE],
            }
        }

        pub fn from(from_grid: [[u8; BOARD_SIZE]; BOARD_SIZE]) -> Board {
            let mut grid: [[Cell; BOARD_SIZE]; BOARD_SIZE] = [[Cell {
                val: 0,
                poss: [false; BOARD_SIZE],
            }; BOARD_SIZE];
                BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    grid[i][j] = Cell {
                        val: from_grid[i][j],
                        poss: [false; BOARD_SIZE],
                    }
                }
            }
            Board { grid }
        }

        pub fn print_board(self) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::board_mod;
    use board_mod::*;

    #[test]
    fn given_ref_to_blank_board_should_print_board() {
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

        let actual = board.print_board();

        assert_eq!(actual, expected);
    }

    #[test]
    fn given_grid_should_build_board_from_it() {
        let poss = [false; BOARD_SIZE];
        let row = [
            Cell { val: 1, poss },
            Cell { val: 2, poss },
            Cell { val: 3, poss },
            Cell { val: 4, poss },
            Cell { val: 5, poss },
            Cell { val: 6, poss },
            Cell { val: 7, poss },
            Cell { val: 8, poss },
            Cell { val: 9, poss },
        ];
        #[rustfmt::skip]
        let given = [
            [1,2,3, 4,5,6, 7,8,9],
            [1,2,3, 4,5,6, 7,8,9],
            [1,2,3, 4,5,6, 7,8,9],
     
            [1,2,3, 4,5,6, 7,8,9],
            [1,2,3, 4,5,6, 7,8,9],
            [1,2,3, 4,5,6, 7,8,9],
     
            [1,2,3, 4,5,6, 7,8,9],
            [1,2,3, 4,5,6, 7,8,9],
            [1,2,3, 4,5,6, 7,8,9],
        ];
        let expected = [row, row, row, row, row, row, row, row, row];

        let actual = Board::from(given);

        assert_eq!(actual.grid, expected);
    }
}
