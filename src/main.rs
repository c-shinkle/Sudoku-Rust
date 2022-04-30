mod algorithm;
mod board;
use algorithm::naive;
use board::Board;

fn main() {
    let mut given = Board::new();
    let result = given.set_board_file("./res/main.txt");
    if result.is_err() {
        eprintln!("Error setting board {:?}", result.err().unwrap());
        return;
    }

    let is_error = naive(&mut given);
    println!("{}", is_error);
    println!("{}", given.print_board());
}
