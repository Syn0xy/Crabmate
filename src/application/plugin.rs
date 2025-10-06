use bevy::prelude::*;

use crabmate_ai_minimax::ChessAIMinimaxPlugin;
use crabmate_ai_random::ChessAIRandomPlugin;
use crabmate_chess::{
    events::{
        DestroyPieceEvent, MoveAttemptEvent, MovePieceEvent, PromotionPieceEvent, SpawnPieceEvent,
        StartChessEvent,
    },
    model::{ChessBoard, ChessTeam},
    plugin::ChessPlugin,
};
use crabmate_interaction::{component::ChessInteraction, plugin::ChessInteractionPlugin};

use crate::{
    AppState,
    application::SelectedPiece,
    constants::application_constants,
    events::{DespawnChessMarkers, SpawnChessMarkers},
};

use super::{
    ChessMarker,
    piece_entity::{PieceEntity, PieceEntityData},
};

const TILE_SIZE: i32 = 32;
const BOARD_OFFSET: IVec2 = IVec2::new(4 * TILE_SIZE - 16, 4 * TILE_SIZE - 16);
const TMP_CHESS_TEAM: &ChessTeam = &ChessTeam::White;

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PieceEntityData::default())
            .insert_resource(SelectedPiece::default())
            .add_event::<SpawnChessMarkers>()
            .add_event::<DespawnChessMarkers>()
            .add_plugins(ChessPlugin)
            .add_plugins(ChessInteractionPlugin)
            .add_plugins(ChessAIRandomPlugin)
            .add_plugins(ChessAIMinimaxPlugin)
            .add_systems(OnEnter(AppState::Game), setup_chess_tiles)
            .add_systems(
                Update,
                (
                    select_piece,
                    play_turn,
                    despawn_chess_markers,
                    spawn_chess_markers,
                    pieces_creation,
                    pieces_destroy,
                    pieces_update,
                    pieces_promotion,
                )
                    .chain()
                    .run_if(in_state(AppState::Game)),
            );
    }
}

fn to_world_position(coord: IVec2, z_index: f32) -> Vec3 {
    (coord * TILE_SIZE - BOARD_OFFSET).as_vec2().extend(z_index)
}

fn setup_chess_tiles(
    mut wevents: EventWriter<StartChessEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for y in 0..8 {
        for x in 0..8 {
            let texture_path = application_constants::get_chess_tile_texture_path(x, y);
            let image = asset_server.load(texture_path);
            let position = IVec2 { x, y };
            commands.spawn((
                Sprite::from_image(image),
                Transform::from_translation(to_world_position(position, -1.0)),
            ));
        }
    }

    let event = StartChessEvent {
        start_team: application_constants::START_TEAM,
    };
    info!("{:?}", event);
    wevents.write(event);
}

fn pieces_creation(
    mut revents: EventReader<SpawnPieceEvent>,
    mut commands: Commands,
    mut pieces_data: ResMut<PieceEntityData>,
    asset_server: Res<AssetServer>,
) {
    for SpawnPieceEvent { id, at, cell } in revents.read() {
        let texture_path = application_constants::get_chess_cell_texture_path(&cell);
        let image = asset_server.load(texture_path);
        let piece_entity = commands
            .spawn((
                PieceEntity { id: *id },
                Sprite::from_image(image),
                Transform::from_translation(to_world_position(at.into(), 0.0)),
            ))
            .id();

        pieces_data.pieces.insert(*id, piece_entity);
    }
}

fn pieces_update(
    mut revents: EventReader<MovePieceEvent>,
    mut pieces_query: Query<&mut Transform>,
    pieces_data: Res<PieceEntityData>,
) {
    for MovePieceEvent { id, to } in revents.read() {
        if let Some(mut transform) = pieces_data
            .pieces
            .get(id)
            .and_then(|&entity| pieces_query.get_mut(entity).ok())
        {
            transform.translation = to_world_position(to.into(), 0.0);
        }
    }
}

fn pieces_destroy(
    mut revents: EventReader<DestroyPieceEvent>,
    mut commands: Commands,
    pieces_data: Res<PieceEntityData>,
) {
    for DestroyPieceEvent { id } in revents.read() {
        if let Some(entity) = pieces_data.pieces.get(id) {
            commands.entity(*entity).despawn();
        }
    }
}

fn pieces_promotion(
    mut revents: EventReader<PromotionPieceEvent>,
    mut piece_sprites_query: Query<&mut Sprite, With<PieceEntity>>,
    asset_server: Res<AssetServer>,
    pieces_data: Res<PieceEntityData>,
    board: Res<ChessBoard>,
) {
    for PromotionPieceEvent { id } in revents.read() {
        let Some(&entity) = pieces_data.pieces.get(id) else {
            return;
        };

        let Some(coord) = board.get_chess_coord(id) else {
            return;
        };

        let Ok(cell) = board.get_cell(coord) else {
            return;
        };

        for sprite in piece_sprites_query.get_mut(entity).iter_mut() {
            let texture_path = application_constants::get_chess_cell_texture_path(cell);
            sprite.image = asset_server.load(texture_path);
        }
    }
}

fn select_piece(
    mut wevents_spawn_markers: EventWriter<SpawnChessMarkers>,
    mut wevents_despawn_markers: EventWriter<DespawnChessMarkers>,
    mut selected_piece: ResMut<SelectedPiece>,
    board: Res<ChessBoard>,
    pieces_query: Query<(&PieceEntity, &ChessInteraction)>,
) {
    for (PieceEntity { id }, interaction) in pieces_query.iter() {
        match interaction {
            ChessInteraction::Pressed => {
                wevents_despawn_markers.write(DespawnChessMarkers);

                if board.play_check(TMP_CHESS_TEAM).is_err() {
                    return;
                }

                if let Some(piece_select_team) = board.get_chess_team(id) {
                    if piece_select_team.eq(TMP_CHESS_TEAM) {
                        selected_piece.id = Some(*id);
                        wevents_spawn_markers.write(SpawnChessMarkers { id: *id });
                    }
                }
            }
            ChessInteraction::Hovered => {}
            ChessInteraction::None => {}
        }
    }
}

fn play_turn(
    mut wevents_move_attempt: EventWriter<MoveAttemptEvent>,
    mut wevents_despawn_markers: EventWriter<DespawnChessMarkers>,
    chess_marker_query: Query<(&ChessMarker, &ChessInteraction)>,
    mut selected_piece: ResMut<SelectedPiece>,
    board: Res<ChessBoard>,
) {
    if selected_piece.id.is_none() {
        return;
    }

    // let team = *TMP_CHESS_TEAM;

    // if let Err(error) = board.play_check(team) {
    //     warn!("{:?}", error);
    //     return;
    // }

    for (&ChessMarker { coord: to }, interaction) in chess_marker_query.iter() {
        match interaction {
            ChessInteraction::Pressed => {
                if let Some(from) = selected_piece
                    .id
                    .take()
                    .and_then(|id| board.get_chess_coord(&id).copied())
                {
                    let team = *TMP_CHESS_TEAM;
                    wevents_move_attempt.write(MoveAttemptEvent { team, from, to });
                    wevents_despawn_markers.write(DespawnChessMarkers);
                }
            }
            ChessInteraction::Hovered => {}
            ChessInteraction::None => {}
        }
    }
}

fn spawn_chess_markers(
    mut commands: Commands,
    mut revents: EventReader<SpawnChessMarkers>,
    asset_server: Res<AssetServer>,
    board: Res<ChessBoard>,
) {
    for SpawnChessMarkers { id } in revents.read() {
        let Some(chess_team) = board.get_chess_team(id) else {
            return;
        };

        let texture_path = application_constants::get_chess_marker_texture_path(chess_team);

        let image = asset_server.load(texture_path);

        if let Some(coord) = board.get_chess_coord(id) {
            if let Ok(chess_moves) = board.get_moves(coord) {
                for chess_move in &chess_moves {
                    commands.spawn((
                        ChessMarker { coord: *chess_move },
                        Sprite::from_image(image.clone()),
                        Transform::from_translation(to_world_position(chess_move.into(), 0.0)),
                    ));
                }
            }
        }
    }
}

fn despawn_chess_markers(
    mut commands: Commands,
    mut revents: EventReader<DespawnChessMarkers>,
    chess_marker_query: Query<Entity, With<ChessMarker>>,
) {
    if revents.is_empty() {
        return;
    }

    revents.clear();

    for chess_marker in chess_marker_query.iter() {
        commands.entity(chess_marker).despawn();
    }
}
