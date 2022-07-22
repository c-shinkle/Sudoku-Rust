use sudoku_rust::board::Board;
use sudoku_rust::iter_combo::combo;

fn main() {
    let mut given = Board::new();
    given
        .set_board_file("./res/bench.txt")
        .expect("File to be present");
    given.set_all_poss();
    combo(&mut given);
    println!("Given: \n{}", given.print_board());
}
