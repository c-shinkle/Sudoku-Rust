use sudoku_rust::board::Board;
use sudoku_rust::fewest_poss::fewest_poss;

fn main() {
    let mut given = Board::new();
    given.set_board_file("./res/bench.txt").expect("File to be present");

    let maybe_solved = fewest_poss(&mut given);
    println!("Did I solve it? {}", maybe_solved.is_some());
    if maybe_solved.is_some() {
        println!("{}", maybe_solved.unwrap().print_board());
    }
}
