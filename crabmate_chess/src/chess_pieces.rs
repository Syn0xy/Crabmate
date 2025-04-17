use bevy::math::UVec2;

#[derive(Debug)]
pub enum ChessPiece {
    Pawn { position: UVec2 },
    Rook { position: UVec2 },
    Knight { position: UVec2 },
    Bishop { position: UVec2 },
    King { position: UVec2 },
    Queen { position: UVec2 },
}
