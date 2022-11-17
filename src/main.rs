use solver::*;
use std::{env, time::Instant};
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
    let mut num_array = num_array_from_text(&board_text);
    let start_time = Instant::now();
    match solve(&mut num_array) {
        SolveStatus::Success => {
            let end_time = Instant::now();
            println!("解けました！ ({:?})", end_time.duration_since(start_time));
            display_board(&num_array);
        }
        SolveStatus::Invalid => {
            println!("不正なデータです。");
        }
        SolveStatus::Duplicated => {
            println!("重複があります。");
        }
        SolveStatus::Unsolvable => {
            println!("解けませんでした。");
        }
    }
}
