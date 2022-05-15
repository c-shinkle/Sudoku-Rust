#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::panic;
    use test::Bencher;

    use sudoku_rust::board::Board;
    use sudoku_rust::combo::combo;
    use sudoku_rust::fewest_poss::fewest_poss;
    use sudoku_rust::naive::naive;
    use sudoku_rust::prev_poss::prev_poss;

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
    fn naive_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved: Option<Board> = Option::default();

        b.iter(|| {
            maybe_solved = naive(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn fewest_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved: Option<Board> = Option::default();

        b.iter(|| {
            maybe_solved = fewest_poss(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn prev_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved: Option<Board> = Option::default();

        b.iter(|| {
            maybe_solved = prev_poss(&mut given);
        });

        teardown(maybe_solved);
    }

    #[bench]
    fn combo_hard_sudoku(b: &mut Bencher) {
        let mut given = setup();
        let mut maybe_solved: Option<Board> = Option::default();

        b.iter(|| {
            maybe_solved = combo(&mut given);
        });

        teardown(maybe_solved);
    }
}
