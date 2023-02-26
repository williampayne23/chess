use crate::board::{Board, Color, Coord, Move, Piece};

pub fn pawn_moves(board: Board, x: isize, y: isize, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    let starting_rank = match piece.color {
        Color::White => 1,
        Color::Black => 6,
    };
    let m = Move {
        from: Coord { x: x, y: y },
        to: Coord {
            x: x,
            y: match piece.color {
                Color::White => y + 1,
                Color::Black => y - 1,
            },
        },
    };
    moves.push(m);
    if y == starting_rank {
        let m = Move {
            from: Coord { x: x, y: y },
            to: Coord {
                x: x,
                y: match piece.color {
                    Color::White => y + 2,
                    Color::Black => y - 2,
                },
            },
        };
        moves.push(m)
    }

    let right_capture_move = Move {
        from: Coord { x: x, y: y },
        to: Coord {
            x: x + 1,
            y: match piece.color {
                Color::White => y + 1,
                Color::Black => y - 1,
            },
        },
    };
    if board
        .get_square(right_capture_move.to.x, right_capture_move.to.y)
        .piece
        .is_some()
    {
        moves.push(right_capture_move);
    }

    if x == 0 {
        return moves;
    }

    let left_capture_move = Move {
        from: Coord { x: x, y: y },
        to: Coord {
            x: x - 1,
            y: match piece.color {
                Color::White => y + 1,
                Color::Black => y - 1,
            },
        },
    };

    if board
        .get_square(left_capture_move.to.x, left_capture_move.to.y)
        .piece
        .is_some()
    {
        moves.push(left_capture_move);
    }

    moves
}

pub fn knight_moves(x: isize, y: isize) -> Vec<Move> {
    let mut moves = Vec::new();

    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x - 1, y: y - 2 },
    });

    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x - 1, y: y + 2 },
    });

    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x + 1, y: y + 2 },
    });

    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x + 1, y: y - 2 },
    });

    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x + 2, y: y + 1 },
    });

    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x + 2, y: y - 1 },
    });

    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x - 2, y: y - 1 },
    });
    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x - 2, y: y + 1 },
    });

    moves
}
