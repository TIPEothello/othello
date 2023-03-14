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
use crate::board::Case;
pub(crate) fn is_legal_move(board: &Vec<Vec<Case>>, bmove: (usize, usize), color: &Case) -> bool {
    let mut adjacents = Vec::<(usize, usize)>::new();
    let mut (i,j) = (bmove-1,bmove-1);
}
