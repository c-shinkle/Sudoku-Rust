pub mod board_mod {

    const BOARD_SIZE: usize = 9;

    pub struct Board {
        pub grid: [[u8; BOARD_SIZE]; BOARD_SIZE],
    }

    impl Board {
        pub fn new() -> Board {
            Board {
                grid: [[0; BOARD_SIZE]; BOARD_SIZE],
            }
        }

        pub fn from(grid: [[u8; BOARD_SIZE]; BOARD_SIZE]) -> Board {
            Board {
                grid,
            }
        }

        pub fn print_board(self: &Board) -> String {
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
    }
}

#[cfg(test)]
mod tests {
    use super::board_mod::Board;

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
        let expected = [
            [1,2,3,4,5,6,7,8,9],
            [1,2,3,4,5,6,7,8,9],
            [1,2,3,4,5,6,7,8,9],
            
            [1,2,3,4,5,6,7,8,9],
            [1,2,3,4,5,6,7,8,9],
            [1,2,3,4,5,6,7,8,9],
            
            [1,2,3,4,5,6,7,8,9],
            [1,2,3,4,5,6,7,8,9],
            [1,2,3,4,5,6,7,8,9],
        ];

        let actual = Board::from(expected);

        assert_eq!(actual.grid, expected);
    }
}
