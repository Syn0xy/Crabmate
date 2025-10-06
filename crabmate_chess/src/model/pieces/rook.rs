use crate::model::{ChessBoard, ChessCoord, ChessTeam};

use super::moves_possibilities;

const ROOK_DIRECTIONS: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

pub(crate) fn get_rook_moves(
    board: &ChessBoard,
    coord: &ChessCoord,
    team: &ChessTeam,
) -> Vec<ChessCoord> {
    moves_possibilities::get_directions_moves(board, coord, team, ROOK_DIRECTIONS)
}
