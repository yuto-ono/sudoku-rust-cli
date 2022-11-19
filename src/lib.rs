use std::fs::File;
use std::io::prelude::*;
mod board;
pub mod constants;
use constants::BOARD_NUM;
pub mod solver;
#[cfg(test)]
mod test;

pub fn read_file(path: &str) -> String {
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

pub fn num_array_from_text(text: &str) -> [u32; BOARD_NUM] {
    let mut num_array = [0; BOARD_NUM];
    let mut i = 0;
    for c in text.chars() {
        if c >= '0' && c <= '9' {
            num_array[i] = (c as u32) - ('0' as u32);
            i += 1;
            if i == BOARD_NUM {
                break;
            }
        }
    }
    num_array
}

pub fn display_board(num_array: &[u32]) {
    for (i, num) in num_array.iter().enumerate() {
        if i % 9 != 8 {
            print!("{} ", num);
        } else {
            println!("{}", num);
        }
    }
}
