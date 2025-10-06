use bevy::prelude::*;
use crabmate_chess::{
    events::{MoveAttemptEvent, PlayRequestEvent},
    model::{ChessBoard, ChessCoord, ChessTeam},
};

const TMP_CHESS_TEAM: &ChessTeam = &ChessTeam::Black;

pub struct ChessAIMinimaxPlugin;

impl Plugin for ChessAIMinimaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_turn);
    }
}

fn play_turn(
    mut revents: EventReader<PlayRequestEvent>,
    mut wevents: EventWriter<MoveAttemptEvent>,
    board: Res<ChessBoard>,
) {
    for PlayRequestEvent { team } in revents.read() {
        if team != TMP_CHESS_TEAM {
            return;
        }

        let boards = Vec::<(ChessBoard, &mut u32)>::default();

        for _ in 0..1 {
            let crnt_board = board.clone();

            let chess_coords = board.get_chess_coords();
        }

        // let event = MoveAttemptEvent {
        //     team: *TMP_CHESS_TEAM,
        //     from,
        //     to,
        // };
        // info!("{:?}", event);
        // wevents.write(event);
    }
}

struct ChessScore {
    score: u32,
}

fn evaluate_score(board: &ChessBoard, team: &ChessTeam) -> ChessScore {
    ChessScore { score: 0 }
}
