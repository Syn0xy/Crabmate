use bevy::prelude::*;
use crabmate_chess::{
    events::{MoveAttemptEvent, PlayRequestEvent},
    model::{ChessBoard, ChessCoord, ChessTeam},
};
use rand::{rng, seq::IndexedRandom};

const TMP_CHESS_TEAM: &ChessTeam = &ChessTeam::Black;

pub struct ChessAIRandomPlugin;

impl Plugin for ChessAIRandomPlugin {
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

        let coords = board
            .get_chess_coords()
            .filter_map(|coord| {
                if board.is_same_team(&coord, TMP_CHESS_TEAM)
                    && matches!(board.get_moves(&coord), Ok(moves) if !moves.is_empty())
                {
                    Some(*coord)
                } else {
                    None
                }
            })
            .collect::<Vec<ChessCoord>>();

        let Some(from) = coords.choose(&mut rng()).copied() else {
            error!("L'IA n'a pas trouver de coordonées a jouer: {:?}", coords);
            return;
        };

        let Ok(Some(to)) = board
            .get_moves(&from)
            .map(|moves| moves.choose(&mut rng()).copied())
        else {
            error!(
                "L'IA n'a pas trouver de movements a jouer sur la coordonnées: {:?}",
                from
            );
            return;
        };

        let event = MoveAttemptEvent {
            team: *TMP_CHESS_TEAM,
            from,
            to,
        };
        info!("{:?}", event);
        wevents.write(event);
    }
}
