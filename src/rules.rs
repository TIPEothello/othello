use crate::board::{Case, DIRECTIONS};

/// Check if a move is legal
/// # Arguments
/// * `board` - The board
/// * `bmove` - The move to check
/// * `color` - The color of the player
/// # Returns
/// * `true` if the move is legal
/// * `false` if the move is illegal
pub fn is_legal_move(board: &[[Case; 8]; 8], bmove: (usize, usize), color: &Case) -> bool {
    if board[bmove.0][bmove.1] != Case::Empty {
        return false;
    }
    for direction in DIRECTIONS {
        if check_direction(board, (bmove.0 as i8, bmove.1 as i8), direction, color) {
            return true;
        }
    }
    false
}

pub fn is_legal_move_with_gain(
    board: &[[Case; 8]; 8],
    bmove: (usize, usize),
    color: &Case,
) -> (bool, usize) {
    if board[bmove.0][bmove.1] != Case::Empty {
        return (false, 0);
    }
    let mut gain = 0;
    for direction in DIRECTIONS {
        let (is_legal, direction_gain) =
            check_direction_with_gain(board, (bmove.0 as i8, bmove.1 as i8), direction, color);
        if is_legal {
            gain += direction_gain;
        }
    }
    (gain > 0, gain)
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
    board: &[[Case; 8]; 8],
    start: (i8, i8),
    direction: (i8, i8),
    color: &Case,
) -> bool {
    let mut x = start.0 + direction.0;
    let mut y = start.1 + direction.1;
    while (0..8).contains(&x)
        && (0..8).contains(&y)
        && board[x as usize][y as usize] == enemy(color)
    {
        x += direction.0;
        y += direction.1;
    }
    (0..8).contains(&x)
        && (0..8).contains(&y)
        && board[x as usize][y as usize] == *color
        && ((x - start.0).abs() > 1 || (y - start.1).abs() > 1)
}

pub fn check_direction_with_gain(
    board: &[[Case; 8]; 8],
    start: (i8, i8),
    direction: (i8, i8),
    color: &Case,
) -> (bool, usize) {
    let mut x = start.0 + direction.0;
    let mut y = start.1 + direction.1;
    let mut gain = 0;
    while (0..8).contains(&x)
        && (0..8).contains(&y)
        && board[x as usize][y as usize] == enemy(color)
    {
        x += direction.0;
        y += direction.1;
        gain += 1;
    }
    (
        (0..8).contains(&x)
            && (0..8).contains(&y)
            && board[x as usize][y as usize] == *color
            && ((x - start.0).abs() > 1 || (y - start.1).abs() > 1),
        gain,
    )
}

pub fn enemy(color: &Case) -> Case {
    match color {
        Case::White => Case::Black,
        Case::Black => Case::White,
        Case::Empty => Case::Empty,
    }
}
