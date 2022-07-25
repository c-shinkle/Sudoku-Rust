use std::sync::atomic::compiler_fence;
use sudoku_rust::board::Board;
use sudoku_rust::iter::combo::combo;

fn main() {
    let mut original = Board::new();
    original
        .set_board_file("./res/bench.txt")
        .expect("File to be present");
    for _ in 0..1 {
        let mut new = original;
        let solution = combo(&mut new).expect("This board is unsolvable!");
        println!("{}", solution.print_board());
    }
}
