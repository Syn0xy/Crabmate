use bevy::prelude::*;

pub trait ChessPlayer: Send + Sync {}

#[derive(Default, Resource)]
pub struct ChessGamePlay {
    pub first: Option<Box<dyn ChessPlayer>>,
    pub second: Option<Box<dyn ChessPlayer>>,
}

pub struct ChessGamePlayPlugin;

impl Plugin for ChessGamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChessGamePlay::default());
    }
}
