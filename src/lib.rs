
// first character in "position" variable is a1, then a2, a3 ... 9th character is b1, etc
// in accordance with FEN notation, white pieces are denoted with UPPERCASE LETTERS and black pieces are denoted with lowercase letters

pub struct Game {
position: Vec<char>,
white_to_move: bool,
}

impl Game {
    fn print_board(&self) {
        /// Prints a visual chessboard in standard output that corresponds to the current position.
        let expanded_board: Vec<char> = self.position.clone();
        let mut board_lines: [String; 8] = ["".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(), ]; // because 
        let mut current_square_number: usize = 0;
        for square in expanded_board {
            board_lines[current_square_number / 8].push(square); // Adds the value of the current square to the String corresponding to the correct line
            current_square_number += 1;
        }
        for line in board_lines.into_iter().rev(){ // reverse the order in which lines appear so they're displayed correctly
            println!("{line}");
        }

}

/* TO DO -------------------------------------------- */


    fn get_rook_moves(&self, pos: usize, white: bool) -> Vec<usize> {
        /// Get the possible squares that a rook at the given position can physically move to. 
        /// Ignores pins (that is, get_rook_moves() also returns rook moves which would leave the king in check)
        let mut rook_moves_result: Vec<usize> = vec![];
        let up_bound: usize = pos % 8 + 57;     
        let down_bound: usize = pos % 8 + 1;         
        let left_bound: usize = 8 * (pos / 8) + 1; 
        let right_bound: usize = 8 * (pos / 8) + 8;


        if(white) {
            for square in pos..right_bound {
            if(square != pos) { // You can't move a piece to its own square! or self.position[square] == "E") {
                if(self.position[square].is_lowercase()) { // you can't move a piece to its own square!!

                    rook_moves_result.push(square); // this is a valid square to move to!
                    break; // but you can't move further in this direction!
            }
                else if(self.position[square] == 'E') {
                        rook_moves_result.push(square); // this is a valid square to move to!
                }
            }
        }
        }
        return rook_moves_result;
    }
/* TO DO -----------------------------------------------*/
}



pub fn new_game() -> Game {
    Game {
        position: "RNBQKBNRPPPPPPPPEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEpppppppprnbqkbnr".to_string().chars().collect::<Vec<_>>(),
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

    #[test]
    fn it_also_works() {
        let game2 = new_game();
        game2.print_board();
    }
}



/* Checklist:

* Game initialization (1/1)

Turn indication incl. move making (1/2)
    * Turn indicator
    - Move making

Move sets (0/6)
    - Rook
    - King
    - Bishop
    - Queen
    - Knight
    - Pawn

Check and pins (0/2)

Promotion (0/4)





*/
