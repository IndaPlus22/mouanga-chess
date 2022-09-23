pub fn new_game() -> Game {
    let mut position = "RNBQKBNRPPPPPPPPEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEpppppppprnbqkbnr";   //  first character is a1, then a2, a3 ... 9th character is b1, etc
    let mut bool white_to_move = true;

#[cfg(test)]
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



