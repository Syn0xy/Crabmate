use std::collections::HashMap;

use bevy::ecs::resource::Resource;

use crate::{
    constants::{BOARD_HEIGHT, BOARD_WIDTH},
    errors::ChessError,
    events::{DestroyPieceEvent, PromotionPieceEvent},
    model::{ChessCell, ChessCoord, ChessTeam, pieces::PieceMoves},
    utils::BijectionMap,
};

use super::{ChessId, ChessPiece};

#[derive(Resource, Debug, Default, Clone)]
pub struct ChessBoard {
    chess_id_sequence: u32,
    current_team_turn: ChessTeam,
    pieces: HashMap<ChessId, ChessCell>,
    coords: BijectionMap<ChessCoord, ChessId>,
}

impl ChessBoard {
    pub const fn next_chess_id(&mut self) -> ChessId {
        let index = self.chess_id_sequence;
        self.chess_id_sequence += 1;
        ChessId { index }
    }

    pub fn set_current_turn(&mut self, team: ChessTeam) {
        self.current_team_turn = team;
    }

    pub fn current_team_turn(&self) -> &ChessTeam {
        &self.current_team_turn
    }

    pub fn set_chess_id_at(&mut self, coord: ChessCoord, chess_id: ChessId) -> bool {
        if self.coords.contains_key_forward(&coord) {
            false
        } else {
            self.coords.insert(coord, chess_id);
            true
        }
    }

    pub fn play_check(&self, team: &ChessTeam) -> Result<(), ChessError> {
        self.current_team_turn
            .eq(team)
            .then_some(())
            .ok_or(ChessError::WrongTeamTurn {
                expect: self.current_team_turn,
                found: *team,
            })
    }

    pub fn switch_turn(&mut self) -> &ChessTeam {
        self.current_team_turn = self.current_team_turn.switch();
        &self.current_team_turn
    }

    pub fn get_chess_id(&self, chess_coord: &ChessCoord) -> Option<&ChessId> {
        self.coords.get_by_forward(chess_coord)
    }

    pub fn get_chess_coord(&self, chess_id: &ChessId) -> Option<&ChessCoord> {
        self.coords.get_by_backward(chess_id)
    }

    pub fn get_chess_team(&self, chess_id: &ChessId) -> Option<&ChessTeam> {
        self.pieces.get(chess_id).map(|cell| cell.get_team())
    }

    pub fn add_pieces(&mut self, chess_id: ChessId, chess_cell: ChessCell) {
        self.pieces.insert(chess_id, chess_cell);
    }

    pub(crate) fn contain_coord(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < BOARD_WIDTH as i32 && y < BOARD_HEIGHT as i32
    }

    pub fn get_chess_ids(&self) -> impl Iterator<Item = &ChessId> {
        self.coords.values_forward()
    }

    pub fn get_chess_coords(&self) -> impl Iterator<Item = &ChessCoord> {
        self.coords.values_backward()
    }

    pub fn get_cell(&self, coord: &ChessCoord) -> Result<&ChessCell, ChessError> {
        self.coords
            .get_by_forward(coord)
            .and_then(|chess_id| self.pieces.get(chess_id))
            .ok_or(ChessError::NoCellFound { coord: *coord })
    }

    pub fn is_empty(&self, chess_coord: &ChessCoord) -> bool {
        self.get_cell(chess_coord).is_err()
    }

    pub fn is_same_team(&self, coord: &ChessCoord, team: &ChessTeam) -> bool {
        self.get_cell(coord)
            .map_or(false, |target_cell| target_cell.team.eq(team))
    }

    pub fn is_other_team(&self, coord: &ChessCoord, team: &ChessTeam) -> bool {
        self.get_cell(coord)
            .map_or(false, |target_cell| !target_cell.team.eq(team))
    }

    pub fn get_moves(&self, coord: &ChessCoord) -> Result<Vec<ChessCoord>, ChessError> {
        self.get_cell(coord)
            .map(|ChessCell { team, piece }| piece.get_moves(self, coord, team))
    }

    pub fn move_to(
        &mut self,
        chess_id: &ChessId,
        target_coord: &ChessCoord,
    ) -> (Option<DestroyPieceEvent>, Option<PromotionPieceEvent>) {
        let Some(coord) = &self.coords.get_by_backward(chess_id).copied() else {
            return (None, None);
        };

        let target_id = self.coords.get_by_forward(target_coord).copied();

        self.coords.remove_by_forward(coord);
        self.coords.insert(*target_coord, *chess_id);

        let mut promotion_event = None;

        if let Some(cell) = self.pieces.get_mut(chess_id) {
            if cell.piece.eq(&ChessPiece::Pawn)
                && target_coord.y
                    == match cell.team {
                        ChessTeam::White => 7,
                        ChessTeam::Black => 0,
                    }
            {
                cell.piece = ChessPiece::Queen;
                promotion_event = Some(PromotionPieceEvent { id: *chess_id });
            }
        }

        (
            target_id.map(|id| DestroyPieceEvent { id }),
            promotion_event,
        )
    }
}
