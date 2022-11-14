use std::env;
use sudoku::*;

const DEFAULT_FILE_NAME: &str = "sample/1.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 {
        &args[1]
    } else {
        DEFAULT_FILE_NAME
    };
    let board_text = read_file(path);
    let board = board_array_from_text(&board_text);
    display_board(board);
}
