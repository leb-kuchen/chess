use crate::*;

impl ChessPiece {
    pub fn new(
        position: ChessSquareCoordinates,
        name: ChessPieceNames,
        player_color: ChessPlayerColor,
    ) -> ChessPiece {
        ChessPiece {
            name,
            position,
            move_directions: Self::set_move_directions(&name),
            player_color,
        }
    }

    fn set_move_directions(piece_name: &ChessPieceNames) -> ChessPieceMoveDirections {
        type Direction = ChessPieceDirectionMoveProperty; // range
                                                          // defaults to `Direction::None`
                                                          // directyl check opponents
        match piece_name {
            ChessPieceNames::King => ChessPieceMoveDirections {
                diagonal: Direction::Limited(1),
                vertical: Direction::Limited(1),
                horizontal: Direction::Limited(1),
                ..Default::default()
            },
            ChessPieceNames::Queen => ChessPieceMoveDirections {
                diagonal: Direction::Unlimited,
                vertical: Direction::Unlimited,
                horizontal: Direction::Unlimited,
                ..Default::default()
            },
            ChessPieceNames::Rook => ChessPieceMoveDirections {
                vertical: Direction::Unlimited,
                horizontal: Direction::Unlimited,
                ..Default::default()
            },
            ChessPieceNames::Bishop => ChessPieceMoveDirections {
                diagonal: Direction::Unlimited,
                ..Default::default()
            },
            ChessPieceNames::Knight => ChessPieceMoveDirections {
                knight: Direction::Knight,
                ..Default::default()
            },
            ChessPieceNames::Pawn => ChessPieceMoveDirections {
                diagonal: Direction::Pawn,
                vertical: Direction::Pawn,
                ..Default::default()
            },
        }
    }

    // 1. check if king is threathend
    // 2. check if the king can be put in safety -> call game_over() || onyl these moveds are valid
    // 3. normal logic
    // 4. only moved that dont thread the king
    // find_valid_squares helper function
    // seperate update function
    // return iterator insted
    pub fn determine_valid_moves(
        &self,
        board: &ChessBoard,
        turn_details: &Turn,
        creation_option: ValidPieceMoveSquaresCreationOptions,
    ) -> ValidPieceMoveSquares {
        type Direction = ChessPieceMoveDirections;
        type DirectionMoveProperty = ChessPieceDirectionMoveProperty;
        let Direction {
            diagonal,
            vertical,
            horizontal,
            knight,
        } = self.move_directions;
        let mut valid_squares = ValidPieceMoveSquaresCreationUtility::new(
            &board.squares_map,
            *turn_details,
            creation_option,
        );
        let unlimited = 10;
        //println!("{:?}", self.move_directions);
        match horizontal {
            DirectionMoveProperty::Unlimited => {
                valid_squares.update(RightSquare(self.position), unlimited);
                valid_squares.update(LeftSquare(self.position), unlimited);
            }
            DirectionMoveProperty::Limited(num) => {
                valid_squares.update(RightSquare(self.position), num.into());
                valid_squares.update(LeftSquare(self.position), num.into());
            }
            _ => (),
        };
        match vertical {
            DirectionMoveProperty::Unlimited => {
                valid_squares.update(UpperSquare(self.position), unlimited);
                valid_squares.update(LowerSquare(self.position), unlimited);
            }
            DirectionMoveProperty::Limited(num) => {
                valid_squares.update(UpperSquare(self.position), num.into());
                valid_squares.update(LowerSquare(self.position), num.into());
            }
            DirectionMoveProperty::Pawn => {
                let pawn_move_range = match turn_details.current_round_number {
                    // match position
                    1 => 2,
                    _ => 1,
                };
                match turn_details.active_player_color {
                    ChessPlayerColor::Black => {
                        valid_squares.update(LowerSquare(self.position), pawn_move_range)
                    }
                    ChessPlayerColor::White => {
                        valid_squares.update(UpperSquare(self.position), pawn_move_range)
                    }
                }
            }
            _ => (),
        };
        // pawn hasnt moved not round number / current_position
        // pawn cant capure forards
        // kings puts himself in danger, move puts own kinger in danger, no valid move
        match diagonal {
            DirectionMoveProperty::Unlimited => {
                valid_squares.update(LowerLeftSquare(self.position), unlimited);
                valid_squares.update(LowerRightSquare(self.position), unlimited);
                valid_squares.update(UpperLeftSquare(self.position), unlimited);
                valid_squares.update(UpperRightSquare(self.position), unlimited);
            }
            DirectionMoveProperty::Limited(num) => {
                valid_squares.update(LowerLeftSquare(self.position), num.into());
                valid_squares.update(LowerRightSquare(self.position), num.into());
                valid_squares.update(UpperLeftSquare(self.position), num.into());
                valid_squares.update(UpperRightSquare(self.position), num.into());
            }

            DirectionMoveProperty::Pawn => match turn_details.active_player_color {
                ChessPlayerColor::Black => {
                    valid_squares.update(LowerRightSquare(self.position), 1); // if let  LowerRightSquare.next()
                    valid_squares.update(LowerLeftSquare(self.position), 1)
                }
                ChessPlayerColor::White => {
                    valid_squares.update(UpperRightSquare(self.position), 1);
                    valid_squares.update(UpperLeftSquare(self.position), 1)
                }
            },

            _ => (),
        }
        match knight {
            _ => (),
        }

        ValidPieceMoveSquares {
            valid_squares: valid_squares.valid_squares,
        }
    }
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self.player_color {
            ChessPlayerColor::Black => match self.name {
                ChessPieceNames::King => "\u{2654}",
                ChessPieceNames::Queen => "\u{2655}",
                ChessPieceNames::Rook => "\u{2656}",
                ChessPieceNames::Bishop => "\u{2657}",
                ChessPieceNames::Knight => "\u{2658}",
                ChessPieceNames::Pawn => "\u{2659}",
            },
            ChessPlayerColor::White => match self.name {
                ChessPieceNames::King => "\u{265A}",
                ChessPieceNames::Queen => "\u{265B}",
                ChessPieceNames::Rook => "\u{265C}",
                ChessPieceNames::Bishop => "\u{265D}",
                ChessPieceNames::Knight => "\u{265E}",
                ChessPieceNames::Pawn => "\u{265F}",
            },
        };
        write!(f, "{message}")
    }
}

impl fmt::Display for ChessPieceNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ChessPieceNames::King => "\u{265A}",
            ChessPieceNames::Queen => "\u{265B}",
            ChessPieceNames::Rook => "\u{265C}",
            ChessPieceNames::Bishop => "\u{265D}",
            ChessPieceNames::Knight => "\u{265E}",
            ChessPieceNames::Pawn => "\u{265F}",
        };
        write!(f, "{message}")
    }
}


impl<'a> ValidPieceMoveSquaresCreationUtility<'a> {
    pub fn new(
        board_squares_map: &'a Map<ChessSquareCoordinates, Option<ChessPiece>>,
        turn_details: Turn,
        creation_option: ValidPieceMoveSquaresCreationOptions,
    ) -> Self {
        Self {
            turn_details,
            board_squares_map,
            valid_squares: Set::new(),
            //   squares_of_direction_matching_king: vec![},
            creation_option,
        }
    }
    pub fn update(
        &mut self,
        direction: impl Iterator<Item = ChessSquareCoordinates>,
        number: usize,
    ) {
        type CreationOptions = ValidPieceMoveSquaresCreationOptions;
        match self.creation_option {
            CreationOptions::InsertAllValidSquaresIntoOneSet => {
                for coordinate in direction.take(number) {
                    let current_square = self.board_squares_map[&coordinate];
                    if let Some(current_square) = current_square {
                        if current_square.player_color != self.turn_details.active_player_color {
                            // if not pawn  straight
                            self.valid_squares.insert(coordinate);
                            return;
                        }
                        return;
                    }
                    self.valid_squares.insert(coordinate);
                }
            }
            CreationOptions::InsertOpponentsValidSquareOfDirection => (),
        };
    }
}
