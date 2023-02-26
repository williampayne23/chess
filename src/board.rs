use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::marker::Copy;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    pub from: Coord,
    pub to: Coord,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece { piece_type, color }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
    pub piece: Option<Piece>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Board {
    current_turn: Color,
    squares: [[Square; 8]; 8],
}

#[allow(dead_code)]
impl Board {
    pub fn get_square(&self, x: isize, y: isize) -> &Square {
        &self.squares[x as usize][y as usize]
    }

    fn set_square(&mut self, x: isize, y: isize, square: Square) {
        self.squares[x as usize][y as usize] = square;
    }

    fn get_piece(&self, x: isize, y: isize) -> Option<Piece> {
        self.get_square(x, y).piece
    }

    fn set_piece(&mut self, x: isize, y: isize, piece: Piece) {
        let mut square = self.get_square(x, y).clone();
        square.piece = Some(piece);
        self.set_square(x, y, square);
    }

    fn remove_piece(&mut self, x: isize, y: isize) {
        let mut square = self.get_square(x, y).clone();
        square.piece = None;
        self.set_square(x, y, square);
    }

    pub fn move_piece(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        let piece = self.get_piece(x1, y1);
        self.remove_piece(x1, y1);
        self.set_piece(x2, y2, piece.unwrap());
    }

    pub fn to_fen_string(&self) -> String {
        let mut fen = String::new();
        let mut empty = 0;
        for y in (0..8).rev() {
            for x in 0..8 {
                match self.get_piece(x, y) {
                    Some(piece) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        match piece.piece_type {
                            PieceType::Pawn => {
                                if piece.color == Color::White {
                                    fen.push('P');
                                } else {
                                    fen.push('p');
                                }
                            }
                            PieceType::Knight => {
                                if piece.color == Color::White {
                                    fen.push('N');
                                } else {
                                    fen.push('n');
                                }
                            }
                            PieceType::Bishop => {
                                if piece.color == Color::White {
                                    fen.push('B');
                                } else {
                                    fen.push('b');
                                }
                            }
                            PieceType::Rook => {
                                if piece.color == Color::White {
                                    fen.push('R');
                                } else {
                                    fen.push('r');
                                }
                            }
                            PieceType::Queen => {
                                if piece.color == Color::White {
                                    fen.push('Q');
                                } else {
                                    fen.push('q');
                                }
                            }
                            PieceType::King => {
                                if piece.color == Color::White {
                                    fen.push('K');
                                } else {
                                    fen.push('k');
                                }
                            }
                        }
                    }
                    None => {
                        empty += 1;
                    }
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
                empty = 0;
            }
            if y > 0 {
                fen.push('/');
            }
        }
        fen.push(' ');
        match self.current_turn {
            Color::White => fen.push('w'),
            Color::Black => fen.push('b'),
        }
        fen
    }

    pub fn board_from_fen_string(fen: String) -> Board {
        let mut board = Board {
            current_turn: Color::White,
            squares: [[Square { piece: None }; 8]; 8],
        };
        let mut x = 0;
        let mut y = 7;
        let bp: Vec<&str> = fen.split(" ").collect();
        let b = bp[0];
        let p = bp[1];
        board.current_turn = match p {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid FEN string"),
        };

        for c in b.chars() {
            if x > 8 || y > 8 {
                panic!("Invalid FEN string");
            }
            match c {
                'P' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::White, PieceType::Pawn));
                    x += 1;
                }
                'N' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::White, PieceType::Knight));
                    x += 1;
                }
                'B' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::White, PieceType::Bishop));
                    x += 1;
                }
                'R' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::White, PieceType::Rook));
                    x += 1;
                }
                'Q' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::White, PieceType::Queen));
                    x += 1;
                }
                'K' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::White, PieceType::King));
                    x += 1;
                }
                'p' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::Black, PieceType::Pawn));
                    x += 1;
                }
                'n' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::Black, PieceType::Knight));
                    x += 1;
                }
                'b' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::Black, PieceType::Bishop));
                    x += 1;
                }
                'r' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::Black, PieceType::Rook));
                    x += 1;
                }
                'q' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::Black, PieceType::Queen));
                    x += 1;
                }
                'k' => {
                    board.squares[x][y].piece = Some(Piece::new(Color::Black, PieceType::King));
                    x += 1;
                }
                '/' => {
                    x = 0;
                    y -= 1;
                }
                '1' => x += 1,
                '2' => x += 2,
                '3' => x += 3,
                '4' => x += 4,
                '5' => x += 5,
                '6' => x += 6,
                '7' => x += 7,
                '8' => x += 8,
                _ => (),
            }
        }
        board
    }

    pub fn construct_board() -> Board {
        Board::board_from_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w".to_string())
    }
}

macro_rules! write_with_error {
    ($f:expr, $fmt:expr) => {
        match write!($f, $fmt) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e)
        }
    };
    ($f:expr, $fmt:expr, $($arg:tt)*) => {
        match write!($f, $fmt, $($arg)*) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e)
        }
    };
}

macro_rules! writeln_with_error {
    ($f:expr, $fmt:expr) => {
        match writeln!($f, $fmt) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e)
        }
    };
    ($f:expr, $fmt:expr, $($arg:tt)*) => {
        match writeln!($f, $fmt, $($arg)*) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e)
        }
    };
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln_with_error!(f, "{} to move", self.current_turn);
        //Print board starting from the 8th rank
        for i in (0..8).rev() {
            for j in 0..8 {
                let Some(Piece {
                    color: c,
                    piece_type: p,
                }) = self.get_square(j, i).piece else {
                    write_with_error!(f, "-");
                    continue;
                };
                let piece = match p {
                    PieceType::Pawn => "P",
                    PieceType::Knight => "N",
                    PieceType::Bishop => "B",
                    PieceType::Rook => "R",
                    PieceType::Queen => "Q",
                    PieceType::King => "K",
                };
                let colored_piece = match c {
                    Color::White => piece.to_uppercase(),
                    Color::Black => piece.to_lowercase(),
                };
                write_with_error!(f, "{}", colored_piece);
            }
            writeln_with_error!(f, "");
        }
        write!(f, "")
    }
}

impl Board {
    fn validate_piece_moves(&self, moves: Vec<Move>) -> Vec<Move> {
        let mut validated_moves: Vec<Move> = Vec::new();
        for m in moves {
            validated_moves.push(m);

            //Moves should not be to squares outside the board
            if m.to.x > 7 || m.to.x < 0 || m.to.y > 7 || m.to.y < 0 {
                validated_moves.pop();
                continue;
            }

            let Some(target_piece) = self.get_square(m.to.x, m.to.y).piece else { continue };
            let Some(piece) = self.get_square(m.from.x, m.from.y).piece else { continue };
            //Moves should not be to pieces of the same color
            if piece.color == target_piece.color {
                validated_moves.pop();
            }
        }
        validated_moves
    }
    #[allow(dead_code)]
    pub fn get_available_moves_for_square(&self, x: isize, y: isize) -> Vec<Move> {
        use crate::piece_moves;
        let Some(piece) = self.get_square(x, y).piece else { return Vec::new() };
        let moves = match piece.piece_type {
            PieceType::Pawn => piece_moves::pawn_moves(*self, x, y, piece),
            PieceType::Knight => piece_moves::knight_moves(x, y),
            _ => Vec::new(),
        };
        self.validate_piece_moves(moves)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn moving_piece() {
        let mut board = super::Board::construct_board();
        board.move_piece(0, 1, 0, 3);
        assert_eq!(
            board.get_square(0, 3).piece.unwrap().piece_type,
            super::PieceType::Pawn
        );
        assert_eq!(board.get_square(0, 1).piece, None);
    }

    #[test]
    fn fen_string() {
        let board = super::Board::board_from_fen_string(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w".to_string(),
        );
        assert_eq!(
            board.get_square(0, 7).piece.unwrap().piece_type,
            super::PieceType::Rook
        );
        assert_eq!(
            board.get_square(0, 7).piece.unwrap().color,
            super::Color::Black
        );
        //Check a white piece
        assert_eq!(
            board.get_square(0, 0).piece.unwrap().piece_type,
            super::PieceType::Rook
        );
        assert_eq!(
            board.get_square(0, 0).piece.unwrap().color,
            super::Color::White
        );
        //Check and empty space
        assert_eq!(board.get_square(3, 3).piece, None);
    }

    #[test]
    fn pawn_moves() {
        //White home pawn
        let board = super::Board::construct_board();
        let moves = board.get_available_moves_for_square(0, 1);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0].to, super::Coord { x: 0, y: 2 });
        assert_eq!(moves[1].to, super::Coord { x: 0, y: 3 });

        //Black home pawn
        let board = super::Board::construct_board();
        let moves = board.get_available_moves_for_square(0, 6);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0].to, super::Coord { x: 0, y: 5 });
        assert_eq!(moves[1].to, super::Coord { x: 0, y: 4 });

        //Test moves for a non home pawn
        let board = super::Board::board_from_fen_string("8/8/8/8/3p4/8/8/8 b".to_string());
        let moves = board.get_available_moves_for_square(3, 3);
        assert_eq!(moves.len(), 1);

        //Test moves for a pawn that can't move
        let board = super::Board::board_from_fen_string("8/8/8/8/3p4/3p4/8/8 b".to_string());
        let moves = board.get_available_moves_for_square(3, 3);
        assert_eq!(moves.len(), 0, "Blocked pawn should have no moves");

        //Capture moves
        let board = super::Board::board_from_fen_string("8/8/8/8/3p4/4P3/8/8 b".to_string());
        let moves = board.get_available_moves_for_square(3, 3);
        assert_eq!(
            moves.len(),
            2,
            "Pawn should have one capture move and one forward move"
        );
    }

    #[test]
    fn knight_moves() {
        let board = super::Board::construct_board();
        let moves = board.get_available_moves_for_square(1, 0);
        assert_eq!(moves.len(), 2);
        //TODO not very robust
        assert_eq!(moves[1].to, super::Coord { x: 2, y: 2 });
        assert_eq!(moves[0].to, super::Coord { x: 0, y: 2 });

        let board = super::Board::board_from_fen_string("8/8/8/8/3n4/8/8/8 b".to_string());
        let moves = board.get_available_moves_for_square(3, 3);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn bishop_moves() {
        let board = super::Board::board_from_fen_string("8/8/8/8/3b4/8/8/8 b".to_string());
        let moves = board.get_available_moves_for_square(3, 3);
        assert_eq!(moves.len(), 13)
    }

    fn rook_moves() {}

    fn queen_moves() {}

    fn king_moves() {}

    #[test]
    fn empty_square_has_no_moves() {
        let board = super::Board::construct_board();
        let moves = board.get_available_moves_for_square(4, 4);
        assert_eq!(moves.len(), 0);
    }
}
