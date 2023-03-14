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
    for i in max(bmove.0 as i32 - 1, 0) as usize..min(bmove.0 + 2, 8) {
        for j in max(bmove.1 as i32 - 1, 0) as usize..min(bmove.1 + 2, 8) {
            if board[i][j] == enemy(color) {
                return true;
            }
        }
    }
    false
}

fn enemy(color: &Case) -> Case {
	match color {
		Case::White => Case::Black,
		Case::Black => Case::White,
		Case::Empty => Case::Empty,
	}
}
