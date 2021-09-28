mod board;

fn main() {
    let b = board::board_mod::Board::new();

    let p = board::board_mod::print_board(&b);

    println!("{}", p);
}
