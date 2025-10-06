use crate::model::{ChessBoard, ChessCoord, ChessPiece, ChessTeam};

use super::{bishop, king, knight, pawn, queen, rook};

pub trait PieceMoves {
    fn get_moves(
        &self,
        board: &ChessBoard,
        coord: &ChessCoord,
        team: &ChessTeam,
    ) -> Vec<ChessCoord>;
}

impl PieceMoves for ChessPiece {
    fn get_moves(
        &self,
        board: &ChessBoard,
        coord: &ChessCoord,
        team: &ChessTeam,
    ) -> Vec<ChessCoord> {
        match self {
            ChessPiece::Pawn => pawn::get_pawn_moves(board, coord, team),
            ChessPiece::Rook => rook::get_rook_moves(board, coord, team),
            ChessPiece::Knight => knight::get_knight_moves(board, coord, team),
            ChessPiece::Bishop => bishop::get_bishop_moves(board, coord, team),
            ChessPiece::King => king::get_king_moves(board, coord, team),
            ChessPiece::Queen => queen::get_queen_moves(board, coord, team),
        }
    }
}
