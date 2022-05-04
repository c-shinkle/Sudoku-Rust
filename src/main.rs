mod algorithm;
mod board;
use algorithm::naive;
use board::Board;

fn main() {
    let mut given = Board::new();
    let result = given.set_board_file("./res/bench.txt");
    if result.is_err() {
        eprintln!("Error setting board {:?}", result.err().unwrap());
        return;
    }

    let maybe_solved = naive(&mut given);
    println!("Did I solve it? {}", maybe_solved.is_some());
    if maybe_solved.is_some() {
        println!("{}", maybe_solved.unwrap().print_board());
    }
}
