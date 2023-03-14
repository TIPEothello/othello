/*
 File: rules.rs
 Created Date: 14 Mar 2023
 Author: realbacon
 -----
 Last Modified: 14/03/2023 05:16:44
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
use std::cmp::{max, min};

use crate::board::Case;
pub fn is_legal_move(board: &Vec<Vec<Case>>, bmove: (usize, usize), color: &Case) -> bool {
    if board[bmove.0][bmove.1] != Case::Empty {
        return false;
    }
	let mut result = false;
	for i in -1..=1 {
		for j in -1..=1 {
			if i == 0 && j == 0 {
				continue;
			}
			result |= check_direction(board, (bmove.0 as i8, bmove.1 as i8), (i, j), color);
		}
	}
	result
	
}

pub fn check_direction(board: &Vec<Vec<Case>>, start: (i8, i8), direction: (i8, i8), color: &Case) -> bool {
	let mut x = start.0 + direction.0;
	let mut y = start.1 + direction.1;
	while x >= 0 && x < 8 && y >= 0 && y < 8 && board[x as usize][y as usize] == enemy(color) {
		x += direction.0;
		y += direction.1;
	}
	x >= 0 && x < 8 && y >= 0 && y < 8 && board[x as usize][y as usize] == *color && ((x - start.0).abs() > 1 || (y - start.1).abs() > 1)
}

fn enemy(color: &Case) -> Case {
	match color {
		Case::White => Case::Black,
		Case::Black => Case::White,
		Case::Empty => Case::Empty,
	}
}
