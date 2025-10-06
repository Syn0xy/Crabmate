use crate::model::{ChessCoord, ChessTeam};

#[derive(Debug)]
pub enum ChessError {
    InvalidCoordinate { coord: ChessCoord },
    NoCellFound { coord: ChessCoord },
    WrongTeamTurn { expect: ChessTeam, found: ChessTeam },
}
