use crate::*;


use terminal_size::terminal_size;

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut chess_board = vec![];
        let mut chess_squares_map = Map::new();
        for row in (1..=8).rev() {
            let mut chess_board_row = vec![];
            for col in 1..=8 {
                let color = match (row % 2 == 0, col % 2 == 0) {
                    (false, false) => ChessSquareColor::Black,
                    (true, false) => ChessSquareColor::White,
                    (false, true) => ChessSquareColor::White,
                    (true, true) => ChessSquareColor::Black,
                };
                let coordinates = ChessSquareCoordinates {
                    row,
                    col: char::from(col + 64),
                };
                chess_squares_map.insert(coordinates, None);
                let square = ChessSquare { color, coordinates };
                chess_board_row.push(square)
            }
            chess_board.push(chess_board_row)
        }
        ChessBoard {
            board_vec: chess_board,
            squares_map: chess_squares_map,
        }
    }
    pub fn set_start_position(&mut self) {
        let front = self.board_vec[..2].iter();
        let back = self.board_vec[6..].iter();
        for (idx, (two_rows, row_num)) in front
            .zip(1_usize..=2)
            .chain(back.zip((1_usize..=2).rev()))
            .enumerate()
        {
            for (square, col_num) in two_rows.iter().zip(1_usize..=8) {
                let ChessSquare { coordinates, .. } = square;
                type Name = ChessPieceNames;
                let chess_piece_name = match (row_num, col_num) {
                    (2, 1..=8) => Name::Pawn,
                    (1, 1 | 8) => Name::Rook,
                    (1, 2 | 7) => Name::Knight,
                    (1, 3 | 6) => Name::Bishop,
                    (1, 4) => Name::Queen,
                    (1, 5) => Name::King,
                    (_, _) => panic!(""),
                };
                let player_color = if idx <= 1 {
                    ChessPlayerColor::Black
                } else {
                    ChessPlayerColor::White
                };
                let chess_piece = ChessPiece::new(*coordinates, chess_piece_name, player_color);
                self.squares_map.insert(*coordinates, Some(chess_piece));
            }
        }
    }


    pub fn draw_chess_board(&self, _turn_details: &Turn) {
        let printed_board = self
            .board_vec
            .iter()
            .map(|row| {
                format!(
                    "{}\n",
                    row.iter()
                        .map(|col| {
                            if let Some(piece) = self.squares_map[&col.coordinates] {
                                format!("{} ", piece)
                            } else {
                                format!("{} ", col.color)
                            }
                        })
                        .collect::<String>()
                )
            })
            .collect::<String>();

    pub fn draw_chess_board(&self, turn_details: &Turn) {
        use either::Either;
        let printed_board = match turn_details.active_player_color {
            ChessPlayerColor::Black => Either::Left(self.board_vec.iter().rev()),
            ChessPlayerColor::White => Either::Right(self.board_vec.iter()),
        }
        .map(|row| {
            let drawn_row = match turn_details.active_player_color {
                ChessPlayerColor::Black => Either::Left(row.iter().rev()),
                ChessPlayerColor::White => Either::Right(row.iter()),
            }
            .map(|col| {
                if let Some(piece) = self.squares_map[&col.coordinates] {
                    format!("{} ", piece)
                } else {
                    format!("{} ", col.color)
                }
            })
            .collect::<String>();
            center_string(
                &drawn_row,
                CenterOptions {
                    fill_with: "-",
                    fill_step: 2,
                },
            ) + "\n"
        })
        .collect::<String>();
        println!("{}", printed_board);
    }
    /*
    pub fn one_move_ahead(chess_board: &ChessBoard,
         first_player: &ChessPlayer,
         second_player: &ChessPlayer,
         turn_details: &Turn,
    ) {
    }
    */
}

// Uses terminal width to measure center
// if width is known at compile time use std::fmt formatter instead
// defaults to the initial size of a termianl if the width and height cannot be determined i.e. 100 and 25
// return empty string if string is zero
pub fn center_string(to_center: &str, center_options: CenterOptions) -> String {
    let CenterOptions {
        fill_with,
        fill_step,
    } = center_options;
    let (width, heigt) =
        terminal_size().unwrap_or((terminal_size::Width(100), terminal_size::Height(25)));
    let (width, _height) = (width.0 as usize, height.0);
    let to_center_len = to_center.chars().count();
    let remaining_space = ((width - to_center_len) / 2) as usize;
    let remaing_space = match fill_step {
        0 => "".to_owned(),
        1 => to_center.repeat(remaining_space),
        _ => (1..=remaining_space)
            .map(|cur| if cur % fill_step == 0 { fill_with } else { " " })
            .collect::<String>(),
    };
    format!("{1}{0}{1}", to_center, remaing_space)
}
pub struct CenterOptions<'a> {
    fill_with: &'a str,
    fill_step: usize,
}
impl<'a> Default for CenterOptions<'a> {
    fn default() -> Self {
        Self {
            fill_with: " ",
            fill_step: 1,
        }
    }
}

impl fmt::Display for ChessSquareColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessSquareColor::White => write!(f, "\u{25A3}"),
            ChessSquareColor::Black => write!(f, "\u{25A2}"),
        }
    }
}
