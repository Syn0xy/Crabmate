use bevy::prelude::*;

use crate::model::{ChessCell, ChessCoord, ChessId, ChessTeam};

#[derive(Event, Debug)]
pub struct StartChessEvent {
    pub start_team: ChessTeam,
}

#[derive(Event, Debug)]
pub struct MoveAttemptEvent {
    pub team: ChessTeam,
    pub from: ChessCoord,
    pub to: ChessCoord,
}

#[derive(Event, Debug)]
pub struct PlayRequestEvent {
    pub team: ChessTeam,
}

#[derive(Event, Debug)]
pub struct SpawnPieceEvent {
    pub id: ChessId,
    pub at: ChessCoord,
    pub cell: ChessCell,
}

#[derive(Event, Debug)]
pub struct MovePieceEvent {
    pub id: ChessId,
    pub to: ChessCoord,
}

#[derive(Event, Debug)]
pub struct DestroyPieceEvent {
    pub id: ChessId,
}

#[derive(Event, Debug)]
pub struct PromotionPieceEvent {
    pub id: ChessId,
}
