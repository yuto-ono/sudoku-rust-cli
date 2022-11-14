use std::fs::File;
use std::io::prelude::*;

pub const BOARD_NUM: usize = 81;

pub fn read_file(path: &str) -> String {
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

pub fn board_array_from_text(text: &str) -> [u32; BOARD_NUM] {
    let mut board: [u32; BOARD_NUM] = [0; BOARD_NUM];
    let mut i = 0;
    for c in text.chars() {
        if c >= '0' && c <= '9' {
            board[i] = (c as u32) - ('0' as u32);
            i += 1;
            if i == BOARD_NUM {
                break;
            }
        }
    }
    board
}

pub fn display_board(board: [u32; BOARD_NUM]) {
    for (i, num) in board.iter().enumerate() {
        if i % 9 != 8 {
            print!("{} ", num);
        } else {
            println!("{}", num);
        }
    }
}
