#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ChessTeam {
    #[default]
    White,
    Black,
}

impl ChessTeam {
    pub fn switch(&self) -> Self {
        match self {
            ChessTeam::White => ChessTeam::Black,
            ChessTeam::Black => ChessTeam::White,
        }
    }
}
