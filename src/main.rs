mod board;

fn main() {
    let b = board::Board::new();

    let p = board::print_board(&b);

    println!("{}", p);
}
