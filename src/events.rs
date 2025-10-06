use bevy::prelude::*;

use crabmate_chess::model::ChessId;

#[derive(Event, Debug)]
pub struct SpawnChessMarkers {
    pub(crate) id: ChessId,
}

#[derive(Event, Debug)]
pub struct DespawnChessMarkers;
