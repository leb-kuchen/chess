// field of the pieces that block in range
// 2048
// impl FromStr seems for ChessSquareCoordinates is more flexible
use core::fmt;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use rand::Rng;
use std::{
	collections::{HashMap as Map, HashSet as Set},
	io,
	str::FromStr,			
};
pub mod chessboard;
pub mod chessgame;
pub mod chesspiece;
pub mod chessplayer;
pub mod coordinates;
pub use crate::{chessboard::*, chessgame::*, chesspiece::*, chessplayer::*, coordinates::*};

// Timer
// Terminal Menu Interface
// write some tests -- confusing as hell

pub fn get_struct() {
    type Coords = ChessSquareCoordinates;
    let a = ChessSquareCoordinates { row: 2, col: 'B' };
    let b = ChessSquareCoordinates { row: 5, col: 'D' };
    let c = ChessSquareCoordinates { row: 4, col: 'D' };
    let d = ChessSquareCoordinates { row: 2, col: 'D' };
    /*
    println!("{:?}", LeftSquare(a).last());
    println!("{:?}", UpperSquare(a).last());
    println!("{:?}", LowerSquare(a).last());
    */
    /*
    let mut test_coords = vec![a, b, c, d];
    test_coords.sort();
    println!("{:?}", test_coords);
    */
    let bottom_left = Coords::from_str("A1").unwrap();
    let bottom_right = Coords::from_str("H1").unwrap();
    let top_left = Coords::from_str("A8").unwrap();
    let top_right = Coords::from_str("H8").unwrap();
}

// Enum InvvaidInputReason
// Has Won / King in Danger



// some professional error handling
/*
enum ChessSquareCreationError {
    ParseIntError(num::ParseIntError),



}
*/
// impl Error trait
#[derive(Debug, PartialEq, Eq)]
pub struct ParseCoordinatesError;
// shift row / col macro ?

impl ChessPlayers {
    pub fn new(black_player: ChessPlayer, white_player: ChessPlayer) -> Self {
        Self {
            black_player,
            white_player,
        }
    }
}
// 1. op tt
// 2: ident row col
// generics ?
// why converto back to char in first place ?
// try no matter what
// struct enum generics ?
// remove redundant
#[derive(Debug)]
pub struct ChessBoard {
    pub board_vec: Vec<Vec<ChessSquare>>,
    pub squares_map: Map<ChessSquareCoordinates, Option<ChessPiece>>,
}
struct ValidPieceMoveSquaresCreationUtility<'a> {
    turn_details: Turn,
    valid_squares: Set<ChessSquareCoordinates>,
    board_squares_map: &'a Map<ChessSquareCoordinates, Option<ChessPiece>>,
    creation_option: ValidPieceMoveSquaresCreationOptions,
}
#[derive(Debug)]
pub struct ValidPieceMoveSquares {
    valid_squares: Set<ChessSquareCoordinates>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ChessSquareColor {
    White,
    Black,
}
#[derive(Debug, Copy, Clone, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChessSquareCoordinates {
    pub row: u8,
    pub col: char,
}
#[derive(Debug, Clone, Copy)]
pub struct ChessSquare {
    pub coordinates: ChessSquareCoordinates,
    pub color: ChessSquareColor,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChessPlayerColor {
    Black,
    White,
}
#[derive(Debug)]
pub struct ChessPlayer {
    player_color: ChessPlayerColor,
    king_position: ChessSquareCoordinates,
    piece_squares_map: Map<ChessSquareCoordinates, ChessPiece>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChessPieceMoveDirections {
    diagonal: ChessPieceDirectionMoveProperty,
    vertical: ChessPieceDirectionMoveProperty,
    horizontal: ChessPieceDirectionMoveProperty,
    knight: ChessPieceDirectionMoveProperty,
}
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub enum ChessPieceDirectionMoveProperty {
    Pawn,
    Unlimited,
    Limited(u8),
    #[default]
    None,
    Knight,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ChessPieceNames {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
pub struct ChessPlayers {
    black_player: ChessPlayer,
    white_player: ChessPlayer,
}


pub enum ValidPieceMoveSquaresCreationOptions {
    InsertAllValidSquaresIntoOneSet,
    InsertOpponentsValidSquareOfDirection,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct ChessPiece {
    name: ChessPieceNames,
    position: ChessSquareCoordinates,
    move_directions: ChessPieceMoveDirections,
    player_color: ChessPlayerColor,
}
#[derive(Debug, Copy, Clone)]
pub struct Turn {
    pub current_round_number: u32,
    pub active_player_color: ChessPlayerColor,
}
pub struct ChessGame {}
