use bevy::prelude::*;
use crabmate_interaction::component::ChessInteraction;

use std::collections::HashMap;

use crabmate_chess::model::ChessId;

#[derive(Component, Debug)]
#[require(ChessInteraction)]
pub(super) struct PieceEntity {
    pub id: ChessId,
}

#[derive(Resource, Default)]
pub(super) struct PieceEntityData {
    pub(super) pieces: HashMap<ChessId, Entity>,
}
