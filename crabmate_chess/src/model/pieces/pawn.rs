use crate::model::{ChessBoard, ChessCoord, ChessTeam};

const fn get_pawn_infos(team: &ChessTeam) -> (i32, u32) {
    match team {
        ChessTeam::White => (1, 1),
        ChessTeam::Black => (-1, 6),
    }
}

pub(crate) fn get_pawn_moves(
    board: &ChessBoard,
    coord: &ChessCoord,
    team: &ChessTeam,
) -> Vec<ChessCoord> {
    let mut moves = Vec::new();
    let (forward, start_row) = get_pawn_infos(team);

    let one_step_x = coord.x as i32;
    let one_step_y = coord.y as i32 + forward;

    let one_step = ChessCoord {
        x: one_step_x as u32,
        y: one_step_y as u32,
    };

    if board.contain_coord(one_step_x, one_step_y) && board.is_empty(&one_step) {
        moves.push(one_step);

        if coord.y == start_row {
            let two_step = ChessCoord {
                x: coord.x,
                y: (coord.y as i32 + 2 * forward) as u32,
            };
            if board.is_empty(&two_step) {
                moves.push(two_step);
            }
        }
    }

    for dx in [-1, 1] {
        let target_x = coord.x as i32 + dx;
        let target_y = coord.y as i32 + forward;

        if !board.contain_coord(target_x, target_y) {
            continue;
        }

        let target_coord = ChessCoord {
            x: target_x as u32,
            y: target_y as u32,
        };

        if board.is_other_team(&target_coord, team) {
            moves.push(target_coord);
        }
    }

    moves
}
