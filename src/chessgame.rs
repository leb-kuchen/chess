use crate::*;
impl ChessGame {
    pub fn menu() {}
    pub fn start_game() {
        println!("Neue Schachrunde, wer fängt an?\nZufall\tSchwarz\tWeiß");
        let mut chess_board = ChessBoard::new();
        chess_board.set_start_position();
        let first_player_color = Self::determine_first_player();
        let black_player = ChessPlayer::new(ChessPlayerColor::Black, &chess_board);
        let white_player = ChessPlayer::new(ChessPlayerColor::White, &chess_board);
        let mut chess_players = ChessPlayers::new(black_player, white_player);
        let mut turn = Turn {
            current_round_number: 1,
            active_player_color: first_player_color,
        };
        for round_number in 1.. {
            turn.current_round_number = round_number;
            println!(
                "Runde {}: {} ist am Zug",
                round_number, turn.active_player_color
            );
            chess_board.draw_chess_board(&turn);
            Self::extract_user_move(&turn, &mut chess_board, &mut chess_players);
            turn.active_player_color = turn.active_player_color.toggle();
        }
    }
    pub fn determine_first_player() -> ChessPlayerColor {
        loop {
            let mut first_player_color = "".to_string();
            io::stdin().read_line(&mut first_player_color).unwrap();
            break match first_player_color.trim().to_uppercase().chars().next() {
                Some('S') => ChessPlayerColor::Black,
                Some('Z') => {
                    if rand::thread_rng().gen_range(1..=2) == 1 {
                        ChessPlayerColor::Black
                    } else {
                        ChessPlayerColor::White
                    }
                }
                Some('W') => ChessPlayerColor::White,
                _ => continue,
            };
        }
    }
    pub fn extract_user_move(
        turn_details: &Turn,
        chess_board: &mut ChessBoard,
        chess_players: &mut ChessPlayers,
    ) {
        let toggled_turn_details = Turn {
            active_player_color: turn_details.active_player_color.toggle(),
            ..*turn_details
        };
        let king_is_threathend = match turn_details.active_player_color {
            ChessPlayerColor::Black => chess_players.black_player.king_is_threathend(
                &chess_players.white_player,
                &chess_board,
                &toggled_turn_details,
            ),
            ChessPlayerColor::White => chess_players.white_player.king_is_threathend(
                &chess_players.black_player,
                &chess_board,
                &toggled_turn_details,
            ),
        };
        // if valid movesquares of all and move square of king
        // the piece can be killed
        // king cannot commit suicide
        // it can be blocked
        // or the king can escape
        //  println!("{:?}", king_is_threathend);
        println!("{:#?}", king_is_threathend);
        if !king_is_threathend.is_empty() {
            println!("Schach");
        }
        'get_user_input: loop {
            let mut player_move_square = "".to_string();
            io::stdin().read_line(&mut player_move_square).unwrap();
            player_move_square = player_move_square.trim().to_uppercase().replace(" ", "");
            let mut player_move_square = player_move_square.chars();
            let mut chess_squares_collection = vec![];
            for _ in 1..=2 {
                let valid_coordinates = match (player_move_square.next(), player_move_square.next())
                {
                    (Some(col @ 'A'..='H'), Some(row @ '1'..='8'))
                    | (Some(row @ '1'..='8'), Some(col @ 'A'..='H')) => ChessSquareCoordinates {
                        row: row.to_digit(10).unwrap() as u8,
                        col,
                    },
                    _ => continue 'get_user_input,
                };
                chess_squares_collection.push(valid_coordinates)
            }
            let current_square = chess_squares_collection[0];
            let new_square = chess_squares_collection[1];
            println!("{} {}", current_square, new_square);

            match chess_board.squares_map[&current_square] {
                Some(mut piece) => {
                    if piece.player_color != turn_details.active_player_color {
                        continue;
                    }
                    // need to check if valid move
                    // insert still valid return else revoke changes
                    // king is threathend logic
                    // make move (n)  logic needs to be seperated // ai can use logic for later to check for win
                    // ai calculate piece score / hashmap seems sensible

                    /*
                        let potencial_valid = piece.get_potencial_moves(
                            &chess_board,
                            &turn_details
                        );
                        println!("{:#?}", potencial_valid);

                    */

                    let valid_squares = piece.determine_valid_moves(
                        &chess_board,
                        &turn_details,
                        ValidPieceMoveSquaresCreationOptions::InsertAllValidSquaresIntoOneSet,
                    );
                    if valid_squares.valid_squares.contains(&new_square) {
                        match turn_details.active_player_color {
                            ChessPlayerColor::Black => {
                                chess_players.black_player.update(piece, new_square)
                            }
                            ChessPlayerColor::White => {
                                chess_players.white_player.update(piece, new_square)
                            }
                        }
                        piece.position = new_square;
                        chess_board.squares_map.insert(new_square, Some(piece));
                        chess_board.squares_map.insert(current_square, None);
                        // new player ?
                        return;
                    } else {
                        println!("TEST {:#?}", valid_squares);
                        continue;
                    }
                }
                None => continue,
            }
        }
    }
}
