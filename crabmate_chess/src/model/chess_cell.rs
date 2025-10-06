use crate::model::{ChessPiece, ChessTeam};

#[derive(Debug, Clone, Copy)]
pub struct ChessCell {
    pub(crate) team: ChessTeam,
    pub(crate) piece: ChessPiece,
}

impl ChessCell {
    pub const fn get_team(&self) -> &ChessTeam {
        &self.team
    }

    pub const fn get_piece(&self) -> &ChessPiece {
        &self.piece
    }
}
