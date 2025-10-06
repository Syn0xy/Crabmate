use crate::model::{ChessBoard, ChessCoord, ChessTeam};

use super::moves_possibilities;

const KING_DIRECTIONS: &[(i32, i32)] = &[
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub(crate) fn get_king_moves(
    board: &ChessBoard,
    coord: &ChessCoord,
    team: &ChessTeam,
) -> Vec<ChessCoord> {
    moves_possibilities::get_directions_moves_2(board, coord, team, KING_DIRECTIONS)
}
