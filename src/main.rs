mod algorithm;
mod board;
use algorithm::naive;
use board::Board;

fn main() {
    let expected_string = "\
    963174258\
    178325649\
    254689731\
    821437596\
    496852317\
    735961824\
    589713462\
    317246985\
    642598173\
    ";
    let mut expected = Board::new();
    expected.set_board_string(expected_string);
    let mut given = Board::new();
    given.set_board_string(
        "\
        003174258\
        178325649\
        254689731\
        821437596\
        496852317\
        735961824\
        589713462\
        317246985\
        042598173\
        ",
    );

    let is_error = naive(&mut given);
    println!("{}", is_error);
    println!("{}", given.print_board());
}
