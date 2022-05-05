#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use sudoku_rust::board::board_mod::Board;
    use sudoku_rust::algorithm::algorithm_mod::naive;

    #[bench]
    fn solve_hard_sudoku(b: &mut Bencher) {
        let mut given = Board::new();
        let result = given.set_board_file("./res/bench.txt");
        if result.is_err() {
            panic!("No file!")
        }
        let mut maybe_solved: Option<Board> = None;

        b.iter(|| {
            maybe_solved = naive(&mut given);
        });

        if maybe_solved.is_none() {
            panic!("No solution!")
        }
    }
}

