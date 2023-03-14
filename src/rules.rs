/*
 File: rules.rs
 Created Date: 14 Mar 2023
 Author: realbacon
 -----
 Last Modified: 14/03/2023 11:02:15
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
use std::cmp::{max, min};

use crate::board::Case;

/// Check if a move is legal
/// # Arguments
/// * `board` - The board
/// * `bmove` - The move to check
/// * `color` - The color of the player
/// # Returns
/// * `true` if the move is legal
/// * `false` if the move is illegal
pub fn is_legal_move(board: &Vec<Vec<Case>>, bmove: (usize, usize), color: &Case) -> bool {
    if board[bmove.0][bmove.1] != Case::Empty {
        return false;
    }
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            if check_direction(board, (bmove.0 as i8, bmove.1 as i8), (i, j), color) {
                return true;
            }
        }
    }
    false
}

/// Check if a move is legal in a given direction
/// # Arguments
/// * `board` - The board
/// * `start` - The starting point of the move
/// * `direction` - The direction of the move
/// * `color` - The color of the player
/// # Returns
/// * `true` if the move is legal in the given direction
/// * `false` if the move is illegal in the given direction
pub fn check_direction(
    board: &Vec<Vec<Case>>,
    start: (i8, i8),
    direction: (i8, i8),
    color: &Case,
) -> bool {
    let mut x = start.0 + direction.0;
    let mut y = start.1 + direction.1;
    while x >= 0 && x < 8 && y >= 0 && y < 8 && board[x as usize][y as usize] == enemy(color) {
        x += direction.0;
        y += direction.1;
    }
    x >= 0
        && x < 8
        && y >= 0
        && y < 8
        && board[x as usize][y as usize] == *color
        && ((x - start.0).abs() > 1 || (y - start.1).abs() > 1)
}

pub fn enemy(color: &Case) -> Case {
    match color {
        Case::White => Case::Black,
        Case::Black => Case::White,
        Case::Empty => Case::Empty,
    }
}
