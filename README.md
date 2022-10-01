# mouanga-chess
the worst chess library ever made. expect bugs, the king taking his own pieces and other fun stuff. this is not chess; in fact, it is an insult to chess and everything it stands for. nothing works. i feel sorry for whoever has to make a gui for this pile of garbage. if you are the fortunate one assigned with such a task, all i can say is good luck, you'll need it. no but like seriously, it's like a child made this code. a cat walking on the keyboard would produce better work. a monkey flinging his poop at the keyboard would probably write a better library than this, this project is the equivalent of a flaming bag of feces. the code contained within this repository is of so low quality that it is directly (and intensely) harmful to human eyes and to the human psyche, comparable to chernobyl but for your eyes. seriously. i suggest you run away as far as you can from this repository, and try to forget it even exists. if you have already been exposed to any part of this source code, you will most likely go insane within 72 hours. it was nice knowing you.

## **-- Game struct --**

contains the fields "position", which is a 64-valued char vector with each index either storing a piece or 'E'
and the self-explanatory booleans white_to_move, white_in_check and black_in_check


## **-- Functions --**

new_game() - Initializes an empty chessboard.

new_game_with_pawns() Initializes a chessboard with black pawns at c5, d5, e5 and white pawns at c3, d3 and e3

new_game_normal() - Initializes a chess board from its starting position.

## **-- Methods --**

`print_board(&self, piece: char, pos: usize, white: bool)`

Prints the current chessboard in standard output, as well as the specified piece's allowed moves (ignoring pins) which are denoted by X. 
The piece must be represented by a capital letter.
Set the piece name to 'E' to just print the chess board.

**Clarification**: it is possible to figure out the allowed moves just by looking at the square. However, this function actually *imagines* a piece at the given square, and prints its theoretical moves.



`get_X_moves(&self, pos: usize, white: bool)`

**Clarification**: This is not a method, but the general name for the methods get_pawn_moves(<args>), get_queen_moves(<args>), etc.

  
Returns a usize vector that contains all of the piece's allowed moves.


  
`get_square_moves(&self, pos: usize)`

Like get_X_moves but you only need to specify a position.


  
`get_piece(&self, pos: usize)`

Returns a char corresponding to the piece at pos.

  
  
`get_king_position(&self, white: bool)`

Returns the king's posititon. The color of said king is decided by the white boolean variable.


  
`make_move(&mut self, pos1: usize, pos2: usize, white: bool)`

Attempts to move the pieec from pos1 to pos2 if it is a legal move and it is <the color of the piece at pos1>'s turn.


  
`get_human_position(&self, pos: usize)`

Attempts to return a human-readable square from a numeric position.
For example, 0 returns A1, 63 returns H8, 27 returns D4.
  
  
## Known issues
  
See [Issues](https://github.com/IndaPlus22/mouanga-chess/issues)
  
*(I can probably fix them but I'm too lazy, maybe I'll do it at 13:14PM on Friday)*
  
