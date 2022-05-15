use sudoku_rust::board::Board;
use sudoku_rust::prev_poss::prev_poss;

fn main() {
    let mut given = Board::new();
    given
        .set_board_file("./res/bench.txt")
        .expect("File to be present");
    given.set_all_poss();
    let maybe_solved = prev_poss(&mut given);
    println!("Did I solve it? {}", maybe_solved.is_some());
    if maybe_solved.is_some() {
        println!("{}", maybe_solved.unwrap().print_board());
    }
}
