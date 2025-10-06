use crate::model::{ChessBoard, ChessCoord, ChessTeam};

use super::moves_possibilities;

const BISHOP_DIRECTIONS: &[(i32, i32)] = &[(1, 1), (1, -1), (-1, 1), (-1, -1)];

pub(crate) fn get_bishop_moves(
    board: &ChessBoard,
    coord: &ChessCoord,
    team: &ChessTeam,
) -> Vec<ChessCoord> {
    moves_possibilities::get_directions_moves(board, coord, team, BISHOP_DIRECTIONS)
}
