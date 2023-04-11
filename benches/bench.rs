#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::panic;
    use test::{black_box, Bencher};

    use sudoku_rust::board::Board;
    use sudoku_rust::{iter, rec};

    fn setup() -> Board {
        let mut given = Board::new();
        given
            .set_board_file("./res/bench.txt")
            .expect("File to be present");
        given
    }

    fn teardown(finished: bool) {
        if !finished {
            panic!("No solution!")
        }
    }

    #[bench]
    fn rec_naive_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut finished = false;

        b.iter(|| {
            finished = black_box(rec::naive::naive(&mut given));
        });

        teardown(finished);
    }

    #[bench]
    fn rec_fewest_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut finished = false;

        b.iter(|| {
            finished = black_box(rec::fewest_poss::fewest_poss(&mut given));
        });

        teardown(finished);
    }

    #[bench]
    fn rec_prev_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut finished = false;

        b.iter(|| {
            finished = black_box(rec::prev_poss::prev_poss(&mut given));
        });

        teardown(finished);
    }

    #[bench]
    fn rec_combo_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut finished = false;

        b.iter(|| {
            finished = black_box(rec::combo::combo(&mut given));
        });

        teardown(finished);
    }

    #[bench]
    fn iter_combo_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut finished = false;

        b.iter(|| {
            finished = black_box(iter::combo::combo(&mut given));
        });

        teardown(finished);
    }
}
