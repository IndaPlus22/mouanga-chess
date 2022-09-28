
// first character in "position" variable is a1, then a2, a3 ... 9th character is b1, etc
// in accordance with FEN notation, white pieces are denoted with UPPERCASE LETTERS and black pieces are denoted with lowercase letters

// initialize constants; not necessary at all but makes it easier to read.
pub struct Piece {
    
}

pub struct Game {
position: String,
white_to_move: bool,
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



/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
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

*/
