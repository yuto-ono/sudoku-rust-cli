use super::*;
use crate::solver::*;

#[test]
fn sample1() {
    let board_text = read_file("sample/1.txt");
    let mut num_array = num_array_from_text(&board_text);
    let answer_text = read_file("sample/1-answer.txt");
    let answer_array = num_array_from_text(&answer_text);
    let solve_status = solve(&mut num_array);
    assert_eq!(solve_status, SolveStatus::Success);
    assert_eq!(num_array, answer_array);
}

#[test]
fn sample2() {
    let board_text = read_file("sample/1.txt");
    let mut num_array = num_array_from_text(&board_text);
    let answer_text = read_file("sample/1-answer.txt");
    let answer_array = num_array_from_text(&answer_text);
    let solve_status = solve(&mut num_array);
    assert_eq!(solve_status, SolveStatus::Success);
    assert_eq!(num_array, answer_array);
}

#[test]
fn invalid_length() {
    let solve_status = solve(&mut [1, 2, 3]);
    assert_eq!(solve_status, SolveStatus::InvalidLength);
}

#[test]
fn no_empty() {
    let board_text = read_file("sample/1-answer.txt");
    let mut num_array = num_array_from_text(&board_text);
    let solve_status = solve(&mut num_array);
    assert_eq!(solve_status, SolveStatus::NoEmpty);
}

#[test]
fn duplicated() {
    let board_text = read_file("sample/duplicated.txt");
    let mut num_array = num_array_from_text(&board_text);
    let solve_status = solve(&mut num_array);
    assert_eq!(solve_status, SolveStatus::Duplicated);
}

#[test]
fn unsolvable() {
    let board_text = read_file("sample/unsolvable.txt");
    let mut num_array = num_array_from_text(&board_text);
    let solve_status = solve(&mut num_array);
    assert_eq!(solve_status, SolveStatus::Unsolvable);
}
