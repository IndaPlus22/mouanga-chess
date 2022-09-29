// first character in "position" string [in new_game(); see below] is a1, then a2, a3 ... 9th character is b1, etc
// in accordance with FEN notation, white pieces are denoted with UPPERCASE LETTERS and black pieces are denoted with lowercase letters

pub struct Game {
    position: Vec<char>,
    white_to_move: bool,
}

impl Game {
    fn print_board(&self, piece: char, pos: usize, white: bool) {
        /// Prints a visual chessboard in standard output that corresponds to the specified piece's allowed moves (ignoring pins).
        /// The name of the piece must be a capital letter, as the color is decided by the white argument.
        /// If the piece is 'E', print the chess board.
        let expanded_board: Vec<char> = self.position.clone();
        let mut allowed_moves: Vec<usize> = vec![];
        let mut board_lines: [String; 8] = [
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ]; // because
        if piece == 'R' {
            allowed_moves = Game::get_rook_moves(self, pos, white);
        }
        else if piece == 'B' {
            allowed_moves = Game::get_bishop_moves(self, pos, white);
        }
        else {
            allowed_moves = vec![];
        }
        let mut current_square_number: usize = 0;
        for square in expanded_board {
            if allowed_moves.contains(&current_square_number) {
                board_lines[current_square_number / 8].push('X')
            }
            else {
            board_lines[current_square_number / 8].push(square); // Adds the value of the current square to the String corresponding to the correct line
            }
            current_square_number += 1; 
        }
        for line in board_lines.into_iter().rev() {
            // reverse the order in which lines appear so they're displayed correctly
            println!("{line}");
        }
    }

    /* TO DO -------------------------------------------- */

    fn get_rook_moves(&self, pos: usize, white: bool) -> Vec<usize> {
        /// Get the possible squares that a rook at the given position can physically move to.
        /// Ignores pins (that is, get_rook_moves() also returns rook moves which would leave the king in check)
        let mut move_result: Vec<usize> = vec![];
        let mut bound_mode: usize = 0;
        let bounds: [usize; 4] = [
            pos % 8 + 57,       // "up direction" bound
            pos % 8,       // down
            8 * (pos / 8), // left
            8 * (pos / 8) + 8,
        ]; // right

        for bound in bounds {
            bound_mode += 1;
            let mut moves_xray: Vec<usize> = vec![];
            if bound_mode == 1 {
                for square in (pos..bound).step_by(8) {
                    moves_xray.push(square);
                }
            } else if bound_mode == 2 {
                for square in (bound..pos).step_by(8) {
                    moves_xray.push(square);
                }
                moves_xray.reverse();
            } else if bound_mode == 3 {
                for square in (bound..pos).step_by(1) {
                    moves_xray.push(square);
                }
                moves_xray.reverse();
            } else if bound_mode == 4 {
                for square in pos..bound {
                    moves_xray.push(square);
                }
            } else {
                panic!("bound_mode out of range, {bound_mode} instead of 0..4")
            }

            for square in moves_xray {
                if square != pos {
                    // You can't move a piece to its own square! {
                    if self.position[square].is_lowercase() {
                        // if true, this is a black piece!
                        if white {
                            move_result.push(square); // this is a capture!
                        }
                        break; // but you can't move further in this direction!
                    } else if self.position[square] == 'E' {
                        move_result.push(square); // this is a valid square to move to!
                    } else if self.position[square].is_uppercase() {
                        // ok, it's a white piece!
                        if !white {
                            move_result.push(square);
                        }
                        break;
                    } else {
                        panic!("invalid piece type at square {square}!")
                    }
                }
            }
        }
        return move_result;
    }




    fn get_bishop_moves(&self, pos: usize, white: bool) -> Vec<usize> {
        /// Get the possible squares that a bishop at the given position can physically move to.
        /// Ignores pins (that is, get_bishop_moves() also returns bishop moves which would leave the king in check)
        let mut move_result: Vec<usize> = vec![];
        let mut bound_mode: usize = 0;
        let mut bounds: [usize; 4] = [62, 63, 0, 1]; // up-left; up-right; down-left; down-right
        let mut moves_xray: Vec<usize> = vec![];
        let mut square: usize = pos;

        loop {
            moves_xray.push(square);
            square += 7;
            if square % 8 == 0 || square >= 56 {
                break;
            }
        }square = pos;

        loop {
            moves_xray.push(square);
            square += 9;
            if square % 8 == 7 || square >= 56 {
                break;
            }
        }square = pos;

        loop {
            moves_xray.push(square);
            if square < 9 {
                break;
            }
            else {
                square -= 9;
                if(square % 8 == 0) {
                    break;
                }
            
            }
            
        }square = pos;

        loop {
            moves_xray.push(square);
            if square < 8 {
                break;
            } 
            else {
                square -= 7;
                if(square % 8 == 7) {
                break;
            }
        }
    }square = pos;
        println!{"moves_xray: {:?}", moves_xray};
        for square in moves_xray {
           if square != pos {
                // You can't move a piece to its own square! {
                if self.position[square].is_lowercase() {
                    // if true, this is a black piece!
                    if white {
                        move_result.push(square); // this is a capture!
                    }
                        break; // but you can't move further in this direction!
                    } else if self.position[square] == 'E' {
                        move_result.push(square); // this is a valid square to move to!
                    } else if self.position[square].is_uppercase() {
                        // ok, it's a white piece!
                        if !white {
                            move_result.push(square);
                        }
                        break;
                    } else {
                        panic!("invalid piece type at square {square}!")
                    }
                }
            }
            return move_result;
        }

    fn get_queen_moves(&self, pos: usize, white: bool) -> Vec<usize> { // add rook and queen moves together
        let mut move_result: Vec<usize> = vec![];
        let mut rook_moves = Game::get_rook_moves(self, pos, white);
        let mut bishop_moves = Game::get_bishop_moves(self, pos, white);
        move_result.append(&mut rook_moves);
        move_result.append(&mut bishop_moves);
        return move_result;
    }
    
    fn get_knight_moves (&self, pos: usize, white: bool) -> Vec<usize> {
        let mut moves_xray: Vec<usize> = vec![];

        if pos < 56 && pos % 8 > 1 && (self.position[pos].is_uppercase() != white) {
            moves_xray.push(pos + 6); // up 1, left 2, cant be done if file < C or rank > 7
        }

        if pos < 56 && pos % 8 < 6  && (self.position[pos].is_uppercase() != white){
            moves_xray.push(pos + 10);                 // U1R2
        }

        if pos < 48 && pos % 8 > 0  && (self.position[pos].is_uppercase() != white){
            moves_xray.push(pos + 15);                 // U2L1
        }

        if pos < 48 && pos % 8 < 7  && (self.position[pos].is_uppercase() != white){
            moves_xray.push(pos + 17);                     // U2R1
        }                 
        
        if pos > 7 && pos % 8 > 1  && (self.position[pos].is_uppercase() != white){  
            moves_xray.push(pos - 10);
        }                       // D1L2

        if pos > 7 && pos % 8 < 6  && (self.position[pos].is_uppercase() != white){
            moves_xray.push(pos - 6);         
        }               // D1R2

        if pos > 15 && pos % 8 > 0  && (self.position[pos].is_uppercase() != white){
            moves_xray.push(pos - 17);                       // D2L1
        }

        if pos > 15 && pos % 8 < 1  && (self.position[pos].is_uppercase() != white){
            moves_xray.push(pos - 15);                     // D2R1 
        }


        return moves_xray;    
        }

    fn get_king_moves(&self, pos: usize, white:bool) -> Vec<usize> {
        let mut moves: Vec<usize> = vec![];

        if pos < 56                     {moves.push(pos + 8);} // up
        if pos > 7                      {moves.push(pos - 8);} // down
        if pos % 8 < 7                  {moves.push(pos + 1);} // right
        if pos % 8 > 0                  {moves.push(pos - 1);} // left
        if pos < 56 && pos % 8 > 0      {moves.push(pos + 7);} // up-left
        if pos < 56 && pos % 8 < 7      {moves.push(pos + 9);} // up-right
        if pos > 7  && pos % 8 > 0      {moves.push(pos - 9);} // down-left
        if pos > 7  && pos % 8 < 7      {moves.push(pos - 7);} // down-right


        return moves;
    }

    fn get_pawn_moves(&self, pos: usize, white: bool) -> Vec<usize> {
        let mut moves: Vec<usize> = vec![];
        if(white) {
         // add forward move
         if self.position[pos + 8] == 'E' {
            moves.push(pos + 8);
        }
        // add double fd move
        if pos > 7 && pos < 15 {
            if self.position[pos + 8] == 'E' && self.position[pos + 16] == 'E' {
            moves.push(pos + 16);
            }
        }

        // add fd-left capture if possible
        if self.position[pos + 7].is_lowercase() && pos % 8 > 0 {
            moves.push(pos + 7)
        }
        // add fd-right capture if possible
        if self.position[pos + 9].is_lowercase() && pos % 8 < 7 {
            moves.push(pos + 9);
        }
    }
        else { // ok we're black!

        // add forward move
        if self.position[pos - 8] == 'E' {
            moves.push(pos - 8);
        }
        // add double fd move
        if pos > 7 && pos < 15 {
            if self.position[pos - 8] == 'E' && self.position[pos - 16] == 'E' {
            moves.push(pos - 16);
            }
        }

        // add fd-left capture if possible
        if self.position[pos - 9].is_lowercase() && pos % 8 > 0 {
            moves.push(pos - 9)
        }
        // add fd-right capture if possible
        if self.position[pos - 7].is_lowercase() && pos % 8 < 7 {
            moves.push(pos - 7);
        }
        
            
        }

        return moves;
    }


    }  


/* TO DO -----------------------------------------------*/

pub fn new_game() -> Game {
    Game {
 //       position: "RNBQKBNRPPPPPPPPEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEpppppppprnbqkbnr"
          position: "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE"
            .to_string()
            .chars()
            .collect::<Vec<_>>(),
        white_to_move: true,
    }
}

pub fn get_square(_s: i8) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("\n");
        let game = new_game();
        game.print_board('E', 15, true);
        println!("\nRegular chessboard for reference (aka test 0)\n")
        //    assert_eq!(result, 4);
    }

// Rook tests with edge cases!
    #[test]
    // rook at a1
    fn rook_test1() {
        println!("\n----- TEST 1 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_rook_moves(0, true));
        game.print_board('R', 0, true);
        println!("\n----- TEST 1 END -----\n")
        
    }
    #[test]
    fn rook_test2() {
        println!("\n----- TEST 2 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_rook_moves(0, false));
        game.print_board('R', 0, false);
        println!("\n----- TEST 2 END -----\n")
        
    }
    // rook at h8
    #[test]
    fn rook_test3() {
        println!("\n----- TEST 3 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_rook_moves(63, true));
        game.print_board('R', 63, true);
        println!("\n----- TEST 3 END -----\n")
        
    }
    #[test]
    fn rook_test4() {
        println!("\n----- TEST 4 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_rook_moves(63, false));
        game.print_board('R', 63, false);
        println!("\n----- TEST 4 END -----\n")
        
    }
    // rook at d4
    #[test]
    fn rook_test5() {
        println!("\n----- TEST 5 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_rook_moves(27, true));
        game.print_board('R', 27, true);
        println!("\n----- TEST 5 END -----\n")
        
    }
    #[test]
    fn rook_test6() {
        println!("\n----- TEST 6 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_rook_moves(27, false));
        game.print_board('R', 27, false);
        println!("\n----- TEST 6 END -----\n")
        
    }

// Bishop tests with edge cases!
    #[test]
    // bishop at a1
    fn bishop_test1() {
        println!("\n----- TEST 1 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_bishop_moves(0, true));
        game.print_board('B', 0, true);
        println!("\n----- TEST 1 END -----\n")
        
    }
    #[test]
    fn bishop_test2() {
        println!("\n----- TEST 2 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_bishop_moves(0, false));
        game.print_board('B', 0, false);
        println!("\n----- TEST 2 END -----\n")
        
    }
    // bishop at h8
    #[test]
    fn bishop_test3() {
        println!("\n----- TEST 3 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_bishop_moves(63, true));
        game.print_board('B', 63, true);
        println!("\n----- TEST 3 END -----\n")
        
    }
    #[test]
    fn bishop_test4() {
        println!("\n----- TEST 4 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_bishop_moves(63, false));
        game.print_board('B', 63, false);
        println!("\n----- TEST 4 END -----\n")
        
    }
    // bishop at d4
    #[test]
    fn bishop_test5() {
        println!("\n----- TEST 5 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_bishop_moves(27, true));
        game.print_board('B', 27, true);
        println!("\n----- TEST 5 END -----\n")
        
    }
    #[test]
    fn bishop_test6() {
        println!("\n----- TEST 6 START -----\n");
        let game = new_game();
        println!("{:?}", game.get_bishop_moves(27, false));
        game.print_board('B', 27, false);
        println!("\n----- TEST 6 END -----\n")
        
}

}

/* Checklist:

* Game initialization (1/1)

Turn indication incl. move making (1/2)
    * Turn indicator
    - Move making

Move sets (4.5/6)
    * Rook
    * King
    * Bishop
    * Queen
    * Knight
    - Pawn

Check and pins (0/2)

Promotion (0/4)




Buglist:
Rook vertical movement


if bound_mode == 1 {
                for square in (pos..bound).step_by(7) {
                    moves_xray.push(square);
                    if square % 8 != 0 && square < 56   { // we're not on the far left or far top sides!
                    }
                    else {
                        break
                    }
                    println!("1: {:?}", moves_xray);
                }
            } else if bound_mode == 2 {
                for square in (pos..bound).step_by(9) {
                    moves_xray.push(square);
                    if square % 8 != 7 && square < 56 { // we're not on the far right or far top sides!
                    }
                    else {
                        break
                    }
                    println!("2: {:?}", moves_xray);
                }
            } else if bound_mode == 3 {
                for square in (bound..pos).step_by(9) {
                    moves_xray.push(square);
                    if (square % 8 != 0 && square > 7) { // we're not on the far left or far bottom sides!
                           
                    }
                    else {
                        break
                    }
                    println!("3: {:?}", moves_xray);
                }
                moves_xray.reverse();
            } else if bound_mode == 4 {
                for square in (bound..pos).step_by(7) {
                    moves_xray.push(square);
                    if (square % 8 != 7 && square > 7)  { // we're not on the far right or far bottom sides!
                         
                    }
                    else {
                        break
                    }
                    println!("4: {:?}", moves_xray);
                }
                moves_xray.reverse();
            }

*/
