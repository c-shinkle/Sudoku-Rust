#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::panic;
    use test::Bencher;

    use sudoku_rust::board::Board;
    use sudoku_rust::iter_combo;
    use sudoku_rust::rec_combo;
    use sudoku_rust::rec_fewest_poss::fewest_poss;
    use sudoku_rust::rec_naive::naive;
    use sudoku_rust::rec_prev_poss::prev_poss;

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
            maybe_solved = naive(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn rec_fewest_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved = Option::default();

        b.iter(|| {
            maybe_solved = fewest_poss(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn rec_prev_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved = Option::default();

        b.iter(|| {
            maybe_solved = prev_poss(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn rec_combo_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved = Option::default();

        b.iter(|| {
            maybe_solved = rec_combo::combo(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn iter_combo_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved = Option::default();

        b.iter(|| {
            maybe_solved = iter_combo::combo(&mut given);
        });

        teardown(maybe_solved);
    }
}
