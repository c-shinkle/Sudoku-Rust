use sudoku_rust::board::Board;
use sudoku_rust::fewest_poss::fewest_poss;
use sudoku_rust::naive::naive;

fn main() {
    let mut given = Board::new();
    let result = given.set_board_file("./res/bench.txt");
    if result.is_err() {
        eprintln!("Error setting board {:?}", result.err().unwrap());
        return;
    }

    let maybe_solved = fewest_poss(&mut given);
    println!("Did I solve it? {}", maybe_solved.is_some());
    if maybe_solved.is_some() {
        println!("{}", maybe_solved.unwrap().print_board());
    }
}
