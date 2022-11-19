use super::board::Board;
use super::constants::BOARD_NUM;

#[derive(Debug, PartialEq)]
pub enum SolveStatus {
    Success,
    InvalidLength,
    NoEmpty,
    Duplicated,
    Unsolvable,
}

pub fn solve(num_array: &mut [u32]) -> SolveStatus {
    if num_array.len() != BOARD_NUM {
        return SolveStatus::InvalidLength; // 配列の長さが違う
    }

    let mut board = Board::new(&num_array);
    if !board.is_valid {
        return SolveStatus::Duplicated; // 重複がある
    }
    if board.empty_len == 0 {
        return SolveStatus::NoEmpty; // 空きマスがない
    }
    if !board.solve() {
        return SolveStatus::Unsolvable; // 解くことができない
    }
    board.output_array(num_array);
    SolveStatus::Success // 解けた
}
