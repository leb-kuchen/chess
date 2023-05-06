use crate::*;

impl ChessPlayer {
    pub fn new(player_color: ChessPlayerColor, chess_board: &ChessBoard) -> Self {
        let piece_squares_map = chess_board
            .squares_map
            .iter()
            .filter_map(|(key, val)| {
                if let Some(piece) = val {
                    if piece.player_color == player_color {
                        return Some((*key, *piece));
                    }
                }
                None
            })
            .collect::<Map<_, _>>();
        let king_position = piece_squares_map
            .iter()
            .find_map(|(key, val)| {
                if val.name == ChessPieceNames::King {
                    Some(key)
                } else {
                    None
                }
            })
            .unwrap();
        Self {
            player_color,
            king_position: *king_position,
            piece_squares_map,
        }
    }

    // remove peace if captured ?
    pub fn update(&mut self, mut piece: ChessPiece, new_square: ChessSquareCoordinates) {
        if piece.name == ChessPieceNames::King {
            self.king_position = new_square
        }
        self.piece_squares_map.remove(&piece.position);
        piece.position = new_square;
        self.piece_squares_map.insert(new_square, piece);
    }

    // piece check if king_is_threathend if this piece is moved
    // more flexbile need to seperate move logic
    // change determine valid moves to vec of squares for each direction

    pub fn king_is_threathend(
        &self,
        opposing_player: &ChessPlayer,
        chess_board: &ChessBoard,
        turn_details: &Turn,
    ) -> Vec<ChessPiece> {
        opposing_player
            .piece_squares_map
            .iter()
            .filter_map(|(key, val)| {
                let valid_squares = val.determine_valid_moves(
                    &chess_board,
                    &turn_details,
                    ValidPieceMoveSquaresCreationOptions::InsertOpponentsValidSquareOfDirection,
                );
                //  println!("{} {} {:#?}", key, val, valid_squares);
                if valid_squares.valid_squares.contains(&self.king_position) {
                    Some(*val)
                } else {
                    None
                }
            })
            .collect()
    }
}
impl fmt::Display for ChessPlayerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessPlayerColor::Black => write!(f, "Schwarz"),
            ChessPlayerColor::White => write!(f, "Weiß"),
        }
    }
}
impl ChessPlayerColor {
    pub fn toggle(&self) -> ChessPlayerColor {
        match self {
            ChessPlayerColor::Black => ChessPlayerColor::White,
            ChessPlayerColor::White => ChessPlayerColor::Black,
        }
    }
}
