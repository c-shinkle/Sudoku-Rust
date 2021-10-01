mod board;

fn main() {
    let board = board::board_mod::Board::new();

    let p = board.print_board();

    println!("{}", p);
}
