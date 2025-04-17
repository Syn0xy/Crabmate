use bevy::prelude::*;

use crate::{ChessPiece, basic::BasicChessBoard};

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_chess_board);
    }
}

fn setup_chess_board(mut commands: Commands) {
    commands.spawn(BasicChessBoard::default());

    let _chess_piece = ChessPiece::Pawn {
        position: UVec2 { x: 0, y: 0 },
    };
}
