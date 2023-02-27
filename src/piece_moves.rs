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

macro_rules! check_capture_or_block {
    ($board:ident, $m:ident, $dir:ident, $moves:ident) => {
        if $m.to.x < 0 || $m.to.x > 7 || $m.to.y < 0 || $m.to.y > 7 {
            $dir = false;
        } else if $board.get_square($m.to.x, $m.to.y).piece.is_some()
            && $board.get_square($m.to.x, $m.to.y).piece.unwrap().color
                != $board.get_square($m.from.x, $m.from.y).piece.unwrap().color
        {
            $moves.push($m);
            $dir = false;
        } else if $board.get_square($m.to.x, $m.to.y).piece.is_some() {
            $dir = false;
        } else {
            $moves.push($m);
        }
    };
}

pub fn bishop_moves(board: Board, x: isize, y: isize) -> Vec<Move> {
    let mut moves = Vec::new();

    let mut ne = true;
    let mut nw = true;
    let mut se = true;
    let mut sw = true;

    for i in 1..8 {
        if ne {
            let m = Move {
                from: Coord { x: x, y: y },
                to: Coord { x: x + i, y: y + i },
            };
            check_capture_or_block!(board, m, ne, moves);
        }
        if nw {
            let m = Move {
                from: Coord { x: x, y: y },
                to: Coord { x: x - i, y: y + i },
            };
            check_capture_or_block!(board, m, nw, moves);
        }
        if se {
            let m = Move {
                from: Coord { x: x, y: y },
                to: Coord { x: x + i, y: y - i },
            };
            check_capture_or_block!(board, m, se, moves);
        }
        if sw {
            let m = Move {
                from: Coord { x: x, y: y },
                to: Coord { x: x - i, y: y - i },
            };
            check_capture_or_block!(board, m, sw, moves);
        }
    }
    moves
}

pub fn rook_moves(board: Board, x: isize, y: isize) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut n = true;
    let mut s = true;
    let mut e = true;
    let mut w = true;

    for i in 1..8 {
        if n {
            let m = Move {
                from: Coord { x: x, y: y },
                to: Coord { x: x, y: y + i },
            };
            check_capture_or_block!(board, m, n, moves);
        }
        if s {
            let m = Move {
                from: Coord { x: x, y: y },
                to: Coord { x: x, y: y - i },
            };
            check_capture_or_block!(board, m, s, moves);
        }
        if e {
            let m = Move {
                from: Coord { x: x, y: y },
                to: Coord { x: x + i, y: y },
            };
            check_capture_or_block!(board, m, e, moves);
        }
        if w {
            let m = Move {
                from: Coord { x: x, y: y },
                to: Coord { x: x - i, y: y },
            };
            check_capture_or_block!(board, m, w, moves);
        }
    }

    moves
}

pub fn queen_moves(board: Board, x: isize, y: isize) -> Vec<Move> {
    let mut moves = Vec::new();

    moves.append(&mut rook_moves(board, x, y));
    moves.append(&mut bishop_moves(board, x, y));

    moves
}

pub fn king_moves(x: isize, y: isize) -> Vec<Move> {
    let mut moves = Vec::new();

    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x + 1, y: y },
    });
    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x - 1, y: y },
    });
    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x, y: y + 1 },
    });
    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x, y: y - 1 },
    });
    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x + 1, y: y + 1 },
    });
    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x - 1, y: y + 1 },
    });
    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x + 1, y: y - 1 },
    });
    moves.push(Move {
        from: Coord { x: x, y: y },
        to: Coord { x: x - 1, y: y - 1 },
    });

    moves
}
