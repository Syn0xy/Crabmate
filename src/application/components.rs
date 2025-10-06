use bevy::prelude::*;
use crabmate_chess::model::{ChessCoord, ChessId};
use crabmate_interaction::component::ChessInteraction;

#[derive(Component, Debug)]
#[require(ChessInteraction)]
pub struct ChessMarker {
    pub(crate) coord: ChessCoord,
}

#[derive(Resource, Default, Debug)]
pub struct SelectedPiece {
    pub(crate) id: Option<ChessId>,
}
