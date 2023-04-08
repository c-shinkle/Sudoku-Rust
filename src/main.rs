use sudoku_rust::board::Board;
use sudoku_rust::iter::combo::combo;

fn main() {
    let mut original = Board::new();
    original
        .set_board_file("./res/bench.txt")
        .expect("File to be present");
    for _ in 0..1 {
        let mut new = original;
        let solution = combo(&mut new);
        if solution {
            println!("{new}");
        } else {
            panic!("No solution!");
        }
    }
}
