use crate::model::{ChessBoard, ChessCoord, ChessTeam};

use super::moves_possibilities;

const KNIGHT_DIRECTIONS: &[(i32, i32)] = &[
    (1, 2),
    (1, -2),
    (2, 1),
    (2, -1),
    (-1, 2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
];

pub(crate) fn get_knight_moves(
    board: &ChessBoard,
    coord: &ChessCoord,
    team: &ChessTeam,
) -> Vec<ChessCoord> {
    moves_possibilities::get_directions_moves_2(board, coord, team, KNIGHT_DIRECTIONS)
}
