// first character in "position" string [in new_game(); see below] is a1, then a2, a3 ... 9th character is b1, etc
// in accordance with FEN notation, white pieces are denoted with UPPERCASE LETTERS and black pieces are denoted with lowercase letters
use std::io;

pub struct Game {
    position: Vec<char>,
    white_to_move: bool,
    white_in_check: bool,
    black_in_check: bool,
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

        else if piece == 'Q' {
            allowed_moves = Game::get_queen_moves(self, pos, white);
        }

        else if piece == 'N' {
            allowed_moves = Game::get_knight_moves(self, pos, white);
        }

        else if piece == 'P' {
            allowed_moves = Game::get_pawn_moves(self, pos, white);
        }

        else if piece == 'K' {
            allowed_moves = Game::get_king_moves(self, pos, white);
        }

        else if piece == 'E' {
            allowed_moves = vec![];
        }

        else {
            panic!("Tried to use invalid piece '{piece}' in print_board method!")
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
        /// First checks for up-left, then up-right, then down-left, then down-right movement.
        let mut move_result: Vec<usize> = vec![];
        let mut moves_xray: Vec<usize> = vec![];
        let mut square: usize = pos;

        loop {
            moves_xray.push(square);
            square += 7;
            if square % 8 == 7 || square > 63 { // should be if square % 8 == 0 || square > 56, but we just added 7 to square...
                break;
            }
        }; square = pos;

        loop {
            moves_xray.push(square);
            square += 9;
            if square % 8 == 0 || square > 63 { // should be if square % 8 == 7 || square > 63, but we just added 9 to square...
                break;
            }
        }; square = pos;

        loop {
            moves_xray.push(square);
            if square < 9 {
                break;
            }
            else {
                square -= 9;
                if square % 8 == 7 {
                    break;
                }
            
            }
            
        }; square = pos;

        loop {
            moves_xray.push(square);
            if square < 7 {
                break;
            } 
            else {
                square -= 7;
                if square % 8 == 0 {
                break;
            }
        }
    }; square = pos;
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

        if pos < 56 && pos % 8 > 1  {
            if self.position[pos + 6] == 'E' || self.position[pos + 6].is_uppercase() != white { // If the target square is empty or has an enemy piece!
            moves_xray.push(pos + 6); // up 1, left 2, cant be done if file < C or rank > 7
            }
        }

        if pos < 56 && pos % 8 < 6{
            if self.position[pos + 10] == 'E' || self.position[pos + 10].is_uppercase() != white {
            moves_xray.push(pos + 10);                 // U1R2
            }
        }

        if pos < 48 && pos % 8 > 0{
            if self.position[pos + 15] == 'E' || self.position[pos + 15].is_uppercase() != white {
                moves_xray.push(pos + 15);
                             }                 // U2L1
        }

        if pos < 48 && pos % 8 < 7{
            if self.position[pos + 17] == 'E' || self.position[pos + 17].is_uppercase() != white {
                moves_xray.push(pos + 17);
                                 }                     // U2R1
        }                 
        
        if pos > 7 && pos % 8 > 1  {  
            if self.position[pos - 10] == 'E' || self.position[pos - 10].is_uppercase() != white {
                moves_xray.push(pos - 10);
            }
        }                       // D1L2

        if pos > 7 && pos % 8 < 6 {
            if self.position[pos - 6] == 'E' || self.position[pos - 6].is_uppercase() != white {
                moves_xray.push(pos - 6);
            }         
        }               // D1R2

        if pos > 15 && pos % 8 > 0  {
            if self.position[pos - 17] == 'E' || self.position[pos - 17].is_uppercase() != white {
                moves_xray.push(pos - 17);
            }                       // D2L1
        }

        if pos > 15 && pos % 8 < 7  {
            if self.position[pos - 15] == 'E' || self.position[pos - 15].is_uppercase() != white {
                moves_xray.push(pos - 15);
            };                     // D2R1 
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
        if white {
         // add forward move
         if self.position[pos + 8] == 'E' {
            moves.push(pos + 8);
        }
        // add double fd move
        if pos > 7 && pos <= 15 {
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
        if pos > 47 && pos <= 55 {
            if self.position[pos - 8] == 'E' && self.position[pos - 16] == 'E' {
            moves.push(pos - 16);
            }
        }

        // add fd-left capture if possible
        if pos >= 9 {
            if self.position[pos - 9].is_uppercase() && pos % 8 > 0 && self.position[pos - 9] != 'E' {
                moves.push(pos - 9)
        }
    }
        // add fd-right capture if possible
        if self.position[pos - 7].is_uppercase() && pos % 8 < 7 && self.position[pos - 7] != 'E' {
            moves.push(pos - 7);
        }
        
            
        }

        return moves;
    }

    fn get_square_moves(&self, pos: usize) -> Vec<usize> {
        /// Returns the moves available to the piece on a given square. If said square is empty, return an empty vector.
        let mut piece_type: char = self.position[pos];
        let white: bool = piece_type.is_uppercase();
        let empty_vector: Vec<usize> = vec![];
        piece_type = piece_type.to_ascii_uppercase();
        
        if piece_type == 'R' {
            return Game::get_rook_moves(self, pos, white);
        }

        else if piece_type == 'B' {
            return Game::get_bishop_moves(self, pos, white);
        }

        else if piece_type == 'N' {
            return Game::get_knight_moves(self, pos, white);
        }

        else if piece_type == 'Q' {
            return Game::get_queen_moves(self, pos, white);
        }

        else if piece_type == 'K' {
            return Game::get_king_moves(self, pos, white);
        }

        else if piece_type == 'P' {
            return Game::get_pawn_moves(self, pos, white);
        }

        else if piece_type == 'E' {
            return empty_vector;
        }

        else {
            panic!("Invalid piece type at get_square_moves!")
        }
    }

    fn get_piece(&self, pos: usize) -> char {
        /// Returns the piece at the given position.
        return self.position[pos];
    }

    fn get_king_position(&self, white: bool) -> usize {
        let mut pos: usize = 0;
        if white {
            for square in self.position.clone() {
                if square == 'K' {
                    break;
                }
            pos += 1;
            }
        }

        else {
            for square in self.position.clone() {
                if square == 'k' {
                    break;
                }
            pos += 1;
            }
        }
        return pos;
    }

    fn make_move(&mut self, pos1: usize, pos2: usize, white:bool) {
        /// Attempts to make a move, invalid moves (that lead to self-check) will be cancelled.
        let piece1: char = Game::get_piece(self, pos1);
        let piece2: char = Game::get_piece(self, pos2);
        let allowed_moves = Game::get_square_moves(self, pos1);
        let mut position_count: usize = 0;
        if piece1 == 'E' || (piece1.is_uppercase() != self.white_to_move) {
            println!("Invalid move; tried to move empty square, or it is not currently that player's turn");
            return ();
        }
        
        

        if allowed_moves.contains(&pos2) {
            if !(piece1 == 'P' && pos2 > 55) && !(piece1 == 'p' && pos2 < 8) {// we aren't promoting
            self.position[pos2] = piece1; // try to move the piece
            self.position[pos1] = 'E';      
            }
            else {      // ok, we're promoting

            if white {
                self.position[pos2] = 'Q' // promote to queen because nobody promotes to anything else let's be be honest
            }
            else {
                self.position[pos2] = 'q'
            }
            self.position[pos1] = 'E';
            }

            while position_count < 64 { // now it's time to loop through all the squares and see if we left ourselves in check!
                let king_position = Game::get_king_position(self, white);
                if (self.position[position_count].is_uppercase() != white && self.position[position_count] != 'E'){ 
                let enemy_moves = Game::get_square_moves(self, position_count);
                    if enemy_moves.contains(&king_position) {
                    self.position[pos1] = piece1; // move it back, we're exposing our king!
                    self.position[pos2] = piece2; // fully undo the move!
                    println!("Oops we can't do that!");
                    self.white_to_move = !self.white_to_move;
                    return ();
                }
                }
                position_count += 1;
            }
            self.white_to_move = !self.white_to_move;
            position_count = 0;
            while position_count < 64 { // now it's time to loop through all the squares and see if we checked the enemy!

                let enemy_king_position = Game::get_king_position(self, !white);
                if (self.position[position_count].is_uppercase() == white && self.position[position_count] != 'E'){ 
                let my_moves = Game::get_square_moves(self, position_count);
                if my_moves.contains(&enemy_king_position) {
                    if white {
                        self.black_in_check = true;
                        println!("Black is in check!")
                    }
                    else {
                        self.white_in_check = true;
                        println!("White is in check!")
                    }
                }
                else {
                    if white {
                        self.black_in_check = false;
                    }

                    else {
                        self.white_in_check = false;
                    }
                }
                }
                position_count += 1;
            }
        }

        else {
            println!("Invalid move!");
        }
        





    }

    fn get_human_square(&self, pos: usize) -> String { // no idea how to do this more efficiently..
        /// Returns a human-readable square from the specified position variable.
        /// Example:                                                0  ->  "A1"
        ///                                                         63 ->  "H8"
        ///                                                         27 ->  "D4"
        let mut file: char = 'I';
        if pos / 8 == 0 {
            let mut file: char = 'A';
        }

        else if pos / 8 == 1 {
            let mut file: char = 'B';
        }

        else if pos / 8 == 2 {
            let mut file: char = 'C';
        }

        else if pos / 8 == 3 {
            let mut file: char = 'D';
        }
        else if pos / 8 == 4 {
            let mut file: char = 'E';
        }

        else if pos / 8 == 5 {
            let mut file: char = 'F';
        }
        else if pos / 8 == 6 {
            let mut file: char = 'G';
        }

        else if pos / 8 == 7 {
            let mut file: char = 'H';
        }

        else {  // invalid position: > 63

        }

        let mut rank = ((pos % 8) as u8 ) as char;

        let mut result: String = "".to_string();
        result.push(file);
        result.push(rank); 
        return result;
    }


    }  


/* TO DO -----------------------------------------------*/

pub fn new_game_normal() -> Game {
     Game {
            position: "RNBQKBNRPPPPPPPPEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEpppppppprnbqkbnr"
 //       position: "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE"
            .to_string()
            .chars()
            .collect::<Vec<_>>(),
        white_to_move: true,
        white_in_check: false,
        black_in_check: false,
    }
}

pub fn new_game_with_pawns() -> Game {
    Game {
        position: "KEEEEEEEEEEEEEEEEEPPPEEEEEEEEEEEEEpppEEEEEEEEEEEEEEEEEEEEEEEEEEk"
        .to_string()
        .chars()
        .collect::<Vec<_>>(),
    white_to_move: true,
    white_in_check: false,
    black_in_check: false, 
    }
}

pub fn new_game() -> Game {
    Game {
            position: "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE"
           .to_string()
           .chars()
           .collect::<Vec<_>>(),
       white_to_move: true,
       white_in_check: false,
       black_in_check: false,
   }
}

pub fn new_game_pin_test() -> Game {
    Game {
            position: "KEEEEEEEREEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEErEEEEEEk"
           .to_string()
           .chars()
           .collect::<Vec<_>>(),
       white_to_move: true,
       white_in_check: false,
       black_in_check: false,
   }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("\n--------------------------------");
        let game = new_game();
        game.print_board('E', 15, true);
        println!("\nRegular chessboard for reference ABOVE (aka test 0)\n--------------------------------")
        //    assert_eq!(result, 4);
    }

// Rook tests with edge cases!
    #[test]
    // rook at a1
    fn rook_test1() {
        println!("\n----- TEST 1 START -----\nRook @ a1\n");
        let game = new_game();
        game.print_board('R', 0, true);
        println!("\n----- TEST 1 END -----\n")
        
    }

    // rook at h8
    #[test]
    fn rook_test2() {
        println!("\n----- TEST 3 START -----\nRook @ h8\n");
        let game = new_game();
        game.print_board('R', 63, true);
        println!("\n----- TEST 3 END -----\n")
        
    }
    // rook at d4
    #[test]
    fn rook_test3() {
        println!("\n----- TEST 5 START -----\nRook @ d4\n");
        let game = new_game();
        game.print_board('R', 27, true);
        println!("\n----- TEST 5 END -----\n")
        
    }

// Bishop tests with edge cases!
    #[test]
    // bishop at a1
    fn bishop_test1() {
        println!("\n----- TEST 1 START -----\nBishop @ a1\n");
        let game = new_game();
        game.print_board('B', 0, true);
        println!("\n----- TEST 1 END -----\n")
        
    }

    // bishop at h8
    #[test]
    fn bishop_test3() {
        println!("\n----- TEST 3 START -----\nBishop @ h8\n");
        let game = new_game();
        game.print_board('B', 63, true);
        println!("\n----- TEST 3 END -----\n")
        
    }

    // bishop at d4
    #[test]
    fn bishop_test5() {
        println!("\n----- TEST 5 START -----\nBishop @ d4\n");
        let game = new_game();
        game.print_board('B', 27, true);
        println!("\n----- TEST 5 END -----\n")
        
    }

// Bishop at a8
#[test]
fn bishop_test7() {
    println!("\n----- TEST 7 START -----\nBishop @ a8\n");
    let game = new_game();
    game.print_board('B', 56, false);
    println!("\n----- TEST 7 END -----\n")

    
    
}

// Bishop at h1
#[test]
fn bishop_test8() {
    println!("\n----- TEST 8 START -----\nBishop @ h1\n");
    let game = new_game();
    game.print_board('B', 7, false);
    println!("\n----- TEST 8 END -----\n")

    
    
}

// Queen at d4 (note: queen move is the same as R+B moves in all cases, and I already know B and R both work)
#[test]
fn queen_test1() {
    println!("\n----- TEST 1 START -----\nQueen @ d4\n");
    let game = new_game();
    game.print_board('Q', 27, true);
    println!("\n----- TEST 1 END -----\n")
}

// King at a1, a8, h1, h8, d4
#[test]
fn king_test1() {
    println!("\n----- TEST 1 START -----\nKing @ a1\n");
    let game = new_game();
    game.print_board('K', 0, true);
    println!("\n----- TEST 1 END -----\n")
}

#[test]
fn king_test2() {
    println!("\n----- TEST 2 START -----\nKing @ a8\n");
    let game = new_game();
    game.print_board('K', 7, true);
    println!("\n----- TEST 2 END -----\n")
}

#[test]
fn king_test3() {
    println!("\n----- TEST 3 START -----\nKing @ h1\n");
    let game = new_game();
    game.print_board('K', 56, true);
    println!("\n----- TEST 3 END -----\n")
}

#[test]
fn king_test4() {
    println!("\n----- TEST 4 START -----\nKing @ h8\n");
    let game = new_game();
    game.print_board('K', 63, true);
    println!("\n----- TEST 4 END -----\n")
}

#[test]
fn king_test5() {
    println!("\n----- TEST 1 START -----\nKing @ d4\n");
    let game = new_game();
    game.print_board('K', 27, true);
    println!("\n----- TEST 1 END -----\n")
}

// Knight at a1, a8, h1, h8, d4
#[test]
fn knight_test1() {
    println!("\n----- TEST 1 START -----\nKnight @ a1\n");
    let game = new_game();
    game.print_board('N', 0, true);
    println!("\n----- TEST 1 END -----\n")
}

#[test]
fn knight_test2() {
    println!("\n----- TEST 2 START -----\nKnight @ a8\n");
    let game = new_game();
    game.print_board('N', 7, true);
    println!("\n----- TEST 2 END -----\n")
}

#[test]
fn knight_test3() {
    println!("\n----- TEST 3 START -----\nKnight @ h1\n");
    let game = new_game();
    game.print_board('N', 56, true);
    println!("\n----- TEST 3 END -----\n")
}

#[test]
fn knight_test4() {
    println!("\n----- TEST 4 START -----\nKnight @ h8\n");
    let game = new_game();
    game.print_board('N', 63, true);
    println!("\n----- TEST 4 END -----\n")
}

#[test]
fn knight_test5() {
    println!("\n----- TEST 5 START -----\nKnight @ d4\n");
    let game = new_game();
    game.print_board('N', 27, true);
    println!("\n----- TEST 5 END -----\n")
}

/// White pawns at a2, d4, a7
#[test]
fn white_pawn_test1() {
    println!("\n----- TEST 1 START -----\nPAWN @ a2\n");
    let game = new_game_with_pawns();
    game.print_board('P', 8, true);
    println!("\n----- TEST 1 END -----\n")
}

#[test]
fn white_pawn_test2() {
    println!("\n----- TEST 2 START -----\nPAWN @ d4\n");
    let game = new_game_with_pawns();
    game.print_board('P', 27, true);
    println!("\n----- TEST 2 END -----\n")
}

#[test]
fn white_pawn_test3() {
    println!("\n----- TEST 3 START -----\nPAWN @ a7\n");
    let game = new_game_with_pawns();
    game.print_board('P', 48, true);
    println!("\n----- TEST 3 END -----\n")
}

/// Black pawns at a7, d4, a2
#[test]
fn black_pawn_test1() {
    println!("\n----- TEST 4 START -----\nPAWN @ a7\n");
    let game = new_game_with_pawns();
    game.print_board('P', 48, false);
    println!("\n----- TEST 4 END -----\n")
}

#[test]
fn black_pawn_test2() {
    println!("\n----- TEST 5 START -----\nPAWN @ d4\n");
    let game = new_game_with_pawns();
    game.print_board('P', 27, false);
    println!("\n----- TEST 5 END -----\n")
}

#[test]
fn black_pawn_test3() {
    println!("\n----- TEST 6 START -----\nPAWN @ a2\n");
    let game = new_game_with_pawns();
    game.print_board('P', 8, false);
    println!("\n----- TEST 6 END -----\n")
}

#[test]
fn zzz_move_test1() {
    println!("\n----- TEST 1 START -----\nCAPTURE TEST\n");
    let mut game = new_game_with_pawns();
    game.print_board('P', 27, false);

    println!("And now, let's make a move!");

    game.make_move(19, 27, true);
    game.print_board('E', 33, false);
    println!("\n----- TEST 1 END -----\n");
}

#[test]
fn zzzz_interactive_chess() {
    println!("\n----- INTERACTIVE CHESS START -----\n");
    let mut game = new_game_normal();
    

    loop {
    let mut input = String::new();
    let mut input2 = String::new();
    let mut usize_input: usize = 0;
    let mut usize_input2: usize = 0;
        println!("Move piece at numerical position:");
        io::stdin().read_line(&mut input).expect("Failed to read line"); // Try to read the user's input
        if input == "exit" {
            break;
        }
        usize_input = input.trim().parse().unwrap();
        println!("\n \n");
        game.print_board(game.get_piece(usize_input).to_ascii_uppercase(), usize_input, game.white_to_move); // Print a board that shows that piece's available moves
        println!("Move where?");
        println!("Allowed moves: {:?}", game.get_square_moves(usize_input));
        io::stdin().read_line(&mut input2).expect("Failed to read line");
        if input == "exit" {
            break;
        }
        usize_input2 = input2.trim().parse().unwrap();
        game.make_move(usize_input, usize_input2, game.white_to_move); // Make the move if possible
        println!("\n \n");
        game.print_board('E', 0, true); // Print the board again
    
    usize_input = input.trim().parse().unwrap();
    usize_input2 = input2.trim().parse().unwrap();

    }

}


}

/* Checklist:

* Game initialization (1/1)

Turn indication incl. move making (2/2)
    * Turn indicator
    * Move making

Move sets (6/6)
    * Rook + Tested
    * King + Tested
    * Bishop + Tested
    * Queen + Tested
    * Knight + Tested
    * Pawn + Tested

Check and pins (1/2)
    * Check
    * Pins
Promotion (1/4)
    * Queen
      Rook
      Bishop
      Knight

*/