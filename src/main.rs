use sudoku_rust::board::Board;
use sudoku_rust::rec::combo::combo;

fn main() {
    let mut given = Board::new();
    given
        .set_board_file("./res/bench.txt")
        .expect("File to be present");
    given.set_all_poss();
    let option = combo(&mut given);
    println!("Given: \n{}", option.unwrap().print_board());
}
