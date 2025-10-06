use crate::model::{ChessCell, ChessCoord, ChessPiece, ChessTeam};

pub(crate) const BOARD_WIDTH: u32 = 8;
pub(crate) const BOARD_HEIGHT: u32 = 8;
pub(crate) const BOARD_PIECES: &[(ChessCoord, ChessCell)] = &[
    // WHITE
    (
        ChessCoord { x: 0, y: 0 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Rook,
        },
    ),
    (
        ChessCoord { x: 1, y: 0 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Knight,
        },
    ),
    (
        ChessCoord { x: 2, y: 0 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Bishop,
        },
    ),
    (
        ChessCoord { x: 3, y: 0 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Queen,
        },
    ),
    (
        ChessCoord { x: 4, y: 0 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::King,
        },
    ),
    (
        ChessCoord { x: 5, y: 0 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Bishop,
        },
    ),
    (
        ChessCoord { x: 6, y: 0 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Knight,
        },
    ),
    (
        ChessCoord { x: 7, y: 0 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Rook,
        },
    ),
    (
        ChessCoord { x: 0, y: 1 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 1, y: 1 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 2, y: 1 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 3, y: 1 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 4, y: 1 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 5, y: 1 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 6, y: 1 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 7, y: 1 },
        ChessCell {
            team: ChessTeam::White,
            piece: ChessPiece::Pawn,
        },
    ),
    // BLACK
    (
        ChessCoord { x: 0, y: 7 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Rook,
        },
    ),
    (
        ChessCoord { x: 1, y: 7 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Knight,
        },
    ),
    (
        ChessCoord { x: 2, y: 7 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Bishop,
        },
    ),
    (
        ChessCoord { x: 3, y: 7 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Queen,
        },
    ),
    (
        ChessCoord { x: 4, y: 7 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::King,
        },
    ),
    (
        ChessCoord { x: 5, y: 7 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Bishop,
        },
    ),
    (
        ChessCoord { x: 6, y: 7 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Knight,
        },
    ),
    (
        ChessCoord { x: 7, y: 7 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Rook,
        },
    ),
    (
        ChessCoord { x: 0, y: 6 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 1, y: 6 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 2, y: 6 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 3, y: 6 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 4, y: 6 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 5, y: 6 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 6, y: 6 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Pawn,
        },
    ),
    (
        ChessCoord { x: 7, y: 6 },
        ChessCell {
            team: ChessTeam::Black,
            piece: ChessPiece::Pawn,
        },
    ),
];
