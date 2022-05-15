#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use sudoku_rust::board::Board;
    use sudoku_rust::fewest_poss::fewest_poss;
    use sudoku_rust::naive::naive;
    use sudoku_rust::prev_poss::prev_poss;
    use test::Bencher;

    #[bench]
    fn naive_hard_sudoku(b: &mut Bencher) {
        let mut given = Board::new();
        given
            .set_board_file("./res/bench.txt")
            .expect("File to be present");
        let mut maybe_solved: Option<Board> = Option::default();

        b.iter(|| {
            maybe_solved = naive(&mut given);
        });

        if maybe_solved.is_none() {
            panic!("No solution!")
        }
    }

    #[bench]
    fn fewest_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = Board::new();
        given
            .set_board_file("./res/bench.txt")
            .expect("File to be present");
        let mut maybe_solved: Option<Board> = Option::default();

        b.iter(|| {
            maybe_solved = fewest_poss(&mut given);
        });

        if maybe_solved.is_none() {
            panic!("No solution!")
        }
    }

    #[bench]
    fn prev_poss_hard_sudoku(b: &mut Bencher) {
        let mut given = Board::new();
        given
            .set_board_file("./res/bench.txt")
            .expect("File to be present");
        let mut maybe_solved: Option<Board> = Option::default();

        b.iter(|| {
            maybe_solved = prev_poss(&mut given);
        });

        if maybe_solved.is_none() {
            panic!("No solution!")
        }
    }
}
