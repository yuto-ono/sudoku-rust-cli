use super::board::Board;
use super::constants::BOARD_NUM;

pub enum SolveStatus {
    Success,
    Invalid,
    Duplicated,
    Unsolvable,
}

pub fn solve(num_array: &mut [u32]) -> SolveStatus {
    if num_array.len() != BOARD_NUM {
        return SolveStatus::Invalid;
    }

    let mut board = Board::new(&num_array);
    if !board.is_valid {
        return SolveStatus::Duplicated;
    }
    // TODO: 空きマス0の場合の例外処
    if !board.solve() {
        return SolveStatus::Unsolvable;
    }
    board.output_array(num_array);
    SolveStatus::Success
}
