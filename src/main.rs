mod board;
use board::board_mod::*;

fn main() {
    let mut board = Board::new();
    if board.set_board_file("given_file_name_should_set_board.txt").is_err() {
        eprintln!("Couldn't open file!");
        return;
    };
    // board.set_board_string(&String::from(
    //     "\
    //     963174258\
    //     178325649\
    //     254689731\
    //     821437596\
    //     496852317\
    //     735961824\
    //     589713462\
    //     317246985\
    //     ",
    //     //642598173\
    // ));

    let p = board.print_board();

    println!("{}", p);
}
