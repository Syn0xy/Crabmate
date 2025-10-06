use crabmate_chess::model::{ChessCell, ChessPiece, ChessTeam};

pub const WINDOW_TITLE: &str = "Crabmate";

pub const START_TEAM: ChessTeam = ChessTeam::White;

pub const fn get_chess_tile_texture_path(x: i32, y: i32) -> &'static str {
    if (x + y) % 2 == 0 {
        "board/tile_black.png"
    } else {
        "board/tile_white.png"
    }
}

pub const fn get_chess_marker_texture_path(team: &ChessTeam) -> &str {
    match team {
        ChessTeam::White => "pieces/white/marker_white.png",
        ChessTeam::Black => "pieces/black/marker_black.png",
    }
}

pub const fn get_chess_cell_texture_path(cell: &ChessCell) -> &str {
    let team = cell.get_team();
    let piece = cell.get_piece();

    match team {
        ChessTeam::White => match piece {
            ChessPiece::Pawn => "pieces/white/pawn_white.png",
            ChessPiece::Rook => "pieces/white/rook_white.png",
            ChessPiece::Knight => "pieces/white/knight_white.png",
            ChessPiece::Bishop => "pieces/white/bishop_white.png",
            ChessPiece::King => "pieces/white/king_white.png",
            ChessPiece::Queen => "pieces/white/queen_white.png",
        },
        ChessTeam::Black => match piece {
            ChessPiece::Pawn => "pieces/black/pawn_black.png",
            ChessPiece::Rook => "pieces/black/rook_black.png",
            ChessPiece::Knight => "pieces/black/knight_black.png",
            ChessPiece::Bishop => "pieces/black/bishop_black.png",
            ChessPiece::King => "pieces/black/king_black.png",
            ChessPiece::Queen => "pieces/black/queen_black.png",
        },
    }
}
