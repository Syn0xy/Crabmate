use crate::model::{ChessBoard, ChessCoord, ChessTeam};

pub(super) fn get_directions_moves(
    board: &ChessBoard,
    coord: &ChessCoord,
    team: &ChessTeam,
    directions: &[(i32, i32)],
) -> Vec<ChessCoord> {
    let mut moves = Vec::new();

    for (dx, dy) in directions {
        let mut x = coord.x as i32;
        let mut y = coord.y as i32;

        loop {
            x += dx;
            y += dy;

            if !board.contain_coord(x, y) {
                break;
            }

            let target_coord = ChessCoord {
                x: x as u32,
                y: y as u32,
            };

            if board.is_empty(&target_coord) {
                moves.push(target_coord);
            } else if !board.is_same_team(&target_coord, team) {
                moves.push(target_coord);
                break;
            } else {
                break;
            }
        }
    }

    moves
}

pub(super) fn get_directions_moves_2(
    board: &ChessBoard,
    coord: &ChessCoord,
    team: &ChessTeam,
    directions: &[(i32, i32)],
) -> Vec<ChessCoord> {
    let mut moves = Vec::new();

    for (dx, dy) in directions {
        let x = coord.x as i32 + dx;
        let y = coord.y as i32 + dy;

        if !board.contain_coord(x, y) {
            continue;
        }

        let target_coord = ChessCoord {
            x: x as u32,
            y: y as u32,
        };

        if board.is_empty(&target_coord) || !board.is_same_team(&target_coord, team) {
            moves.push(target_coord);
        }
    }

    moves
}
