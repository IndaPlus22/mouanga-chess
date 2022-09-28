
// first character in "position" variable is a1, then a2, a3 ... 9th character is b1, etc
// in accordance with FEN notation, white pieces are denoted with UPPERCASE LETTERS and black pieces are denoted with lowercase letters
pub struct Piece {

}

pub struct Game {
position: String,
white_to_move: bool,
}

impl Game {
    fn print_board(&self) {
        /// Prints a visual chessboard in standard output that corresponds to the current position.
        let expanded_board: Vec<char> = self.position.chars().collect::<Vec<_>>();
        let mut board_lines: [String; 8] = ["".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(), ];
        let mut current_square_number: usize = 0;
        for square in expanded_board {
            board_lines[current_square_number % 8].push(square); // Adds the value current square to the String corresponding to the correct line
            current_square_number += 1;
        }
        for line in board_lines{
            println!("{line}");
        }

}

}

pub fn new_game() -> Game {
    Game {
        position: "RNBQKBNRPPPPPPPPEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEpppppppprnbqkbnrpppppppp".to_string(),
        white_to_move: true
    }    
}




pub fn get_square(s: i8) -> String {
    todo!();
}



 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let game = new_game();
        game.print_board();
    //    assert_eq!(result, 4);
    }
}



/* Functions to add:
get_rook_moves(<true/false>) - returns legal white rook moves if true, otherwise returns legal black rook moves
                               add this for every piece

print_board() - prints 8 rows like the following:       [ ]  [R]  [Q]  [p]  [ ]  [p]  [K]  [ ]

                                                        [ ]  [R]  [Q]  [p]  [ ]  [p]  [K]  [ ]

                                                        [ ]  [R]  [Q]  [p]  [ ]  [p]  [K]  [ ]

                                                        [ ]  [R]  [Q]  [p]  [ ]  [p]  [K]  [ ]

                                                        [ ]  [R]  [Q]  [p]  [ ]  [p]  [K]  [ ]

                                                        [ ]  [R]  [Q]  [p]  [ ]  [p]  [K]  [ ]

                                                        [ ]  [R]  [Q]  [p]  [ ]  [p]  [K]  [ ]

                                                        [ ]  [R]  [Q]  [p]  [ ]  [p]  [K]  [ ]


*/
