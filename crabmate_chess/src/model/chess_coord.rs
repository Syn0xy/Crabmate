use bevy::math::IVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChessCoord {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl From<&ChessCoord> for IVec2 {
    fn from(&ChessCoord { x, y }: &ChessCoord) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl From<&IVec2> for ChessCoord {
    fn from(&IVec2 { x, y }: &IVec2) -> Self {
        Self {
            x: x as u32,
            y: y as u32,
        }
    }
}
