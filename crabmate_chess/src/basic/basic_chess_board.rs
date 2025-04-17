use bevy::prelude::*;

use crate::ChessBoard;

#[derive(Debug, Component)]
pub struct BasicChessBoard {
    pub width: usize,
    pub height: usize,
}

impl Default for BasicChessBoard {
    fn default() -> Self {
        Self {
            width: 8,
            height: 8,
        }
    }
}

impl ChessBoard for BasicChessBoard {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}
