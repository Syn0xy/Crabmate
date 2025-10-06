use bevy::prelude::*;

use crate::{
    constants::BOARD_PIECES,
    events::{
        DestroyPieceEvent, MoveAttemptEvent, MovePieceEvent, PlayRequestEvent, PromotionPieceEvent,
        SpawnPieceEvent, StartChessEvent,
    },
    model::ChessBoard,
};

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChessBoard::default())
            .add_event::<StartChessEvent>()
            .add_event::<MoveAttemptEvent>()
            .add_event::<PlayRequestEvent>()
            .add_event::<SpawnPieceEvent>()
            .add_event::<MovePieceEvent>()
            .add_event::<DestroyPieceEvent>()
            .add_event::<PromotionPieceEvent>()
            .add_systems(Update, (setup_board, update_plays));
    }
}

fn setup_board(
    mut revents_start: EventReader<StartChessEvent>,
    mut wevents_play_request: EventWriter<PlayRequestEvent>,
    mut wevents_spawn_piece: EventWriter<SpawnPieceEvent>,
    mut board: ResMut<ChessBoard>,
) {
    for &StartChessEvent { start_team: team } in revents_start.read() {
        for &(at, cell) in BOARD_PIECES {
            let id = board.next_chess_id();

            board.set_chess_id_at(at, id);
            board.add_pieces(id, cell);
            let event = SpawnPieceEvent { id, at, cell };
            info!("{:?}", event);
            wevents_spawn_piece.write(event);
        }

        board.set_current_turn(team);
        let event = PlayRequestEvent { team };
        info!("{:?}", event);
        wevents_play_request.write(event);
    }
}

fn update_plays(
    mut revents: EventReader<MoveAttemptEvent>,
    mut wevents_move_piece: EventWriter<MovePieceEvent>,
    mut wevents_destroy_piece: EventWriter<DestroyPieceEvent>,
    mut wevents_promotion_piece: EventWriter<PromotionPieceEvent>,
    mut wevents_play_request: EventWriter<PlayRequestEvent>,
    mut board: ResMut<ChessBoard>,
) {
    for MoveAttemptEvent { team, from, to } in revents.read() {
        if let Err(chess_error) = board.play_check(team) {
            warn!("{:?}", chess_error);
            return;
        }

        let Some(id) = &board.get_chess_id(from).copied() else {
            return;
        };

        if let Some(from_team) = board.get_chess_team(id) {
            if !from_team.eq(team) {
                return;
            }
        }

        let (destroy_event_opt, promotion_event_opt) = board.move_to(id, to);

        destroy_event_opt.map(|event| {
            info!("{:?}", event);
            wevents_destroy_piece.write(event);
        });

        promotion_event_opt.map(|event| {
            info!("{:?}", event);
            wevents_promotion_piece.write(event);
        });

        let event = MovePieceEvent { id: *id, to: *to };
        info!("{:?} -> {:?}", &team, &event);
        wevents_move_piece.write(event);

        let team = *board.switch_turn();

        let event = PlayRequestEvent { team };
        info!("{:?}", event);
        wevents_play_request.write(event);
    }
}
