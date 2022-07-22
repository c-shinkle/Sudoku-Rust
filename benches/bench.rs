#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::panic;
    use test::Bencher;

    use sudoku_rust::board::Board;
    use sudoku_rust::{iter, rec};

    fn setup() -> Board {
        let mut given = Board::new();
        given
            .set_board_file("./res/bench.txt")
            .expect("File to be present");
        given
    }

    fn teardown(board: Option<Board>) {
        if board.is_none() {
            panic!("No solution!")
        }
    }

    #[bench]
    fn rec_naive_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved = Option::default();

        b.iter(|| {
            maybe_solved = rec::naive::naive(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn rec_fewest_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved = Option::default();

        b.iter(|| {
            maybe_solved = rec::fewest_poss::fewest_poss(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn rec_prev_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved = Option::default();

        b.iter(|| {
            maybe_solved = rec::prev_poss::prev_poss(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn rec_combo_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved = Option::default();

        b.iter(|| {
            maybe_solved = rec::combo::combo(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn iter_combo_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved = Option::default();

        b.iter(|| {
            maybe_solved = iter::combo::combo(&mut given);
        });

        teardown(maybe_solved);
    }
}
