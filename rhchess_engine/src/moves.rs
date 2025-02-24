use crate::board;
use crate::board::Board;
use crate::board::Square;

#[derive(Clone, Copy, Debug)]
pub enum Move {
    /// Castle(king_side)
    Castle(bool),
    /// EnPassent(src)
    EnPassent(Square),
    /// Move(rev, dst, src)
    Move(bool, Square, Square),
}

fn knight(board: &Board, src: Square) -> Vec<Move> {
    [
        src.rank
            .checked_sub(2)
            .and_then(|x| Square::new(src.file + 1, x)),
        src.rank
            .checked_sub(1)
            .and_then(|x| Square::new(src.file + 2, x)),
        Square::new(src.file + 2, src.rank + 1),
        Square::new(src.file + 1, src.rank + 2),
        src.file
            .checked_sub(1)
            .and_then(|x| Square::new(x, src.rank + 2)),
        src.file
            .checked_sub(2)
            .and_then(|x| Square::new(x, src.rank + 1)),
        src.file
            .checked_sub(2)
            .and_then(|f| src.rank.checked_sub(1).and_then(|r| Square::new(f, r))),
        src.file
            .checked_sub(1)
            .and_then(|f| src.rank.checked_sub(2).and_then(|r| Square::new(f, r))),
    ]
    .iter()
    .filter_map(|&x| {
        let x = x?;
        match board.get_piece(x) {
            Some(piece) => {
                if piece.owner != board.turn {
                    Some(Move::Move(false, x, src))
                } else {
                    None
                }
            }
            None => Some(Move::Move(true, x, src)),
        }
    })
    .collect()
}

fn to_moves(board: &Board, src: Square, line: impl Iterator<Item = Option<Square>>) -> Vec<Move> {
    let mut ret = Vec::new();
    for i in line {
        match i {
            None => break,
            Some(s) => match board.get_piece(s) {
                None => ret.push(Move::Move(true, s, src)),
                Some(piece) => {
                    if board.turn != piece.owner {
                        ret.push(Move::Move(false, s, src));
                    }
                    break;
                }
            },
        }
    }
    ret
}

fn bishop(board: &Board, src: Square) -> Vec<Move> {
    [
        to_moves(board, src, (1..8).map(|x| src.translate(-x, -x))),
        to_moves(board, src, (1..8).map(|x| src.translate(-x, x))),
        to_moves(board, src, (1..8).map(|x| src.translate(x, -x))),
        to_moves(board, src, (1..8).map(|x| src.translate(x, x))),
    ]
    .concat()
}

fn rook(board: &Board, src: Square) -> Vec<Move> {
    [
        to_moves(board, src, (1..8).map(|x| src.translate(x, 0))),
        to_moves(board, src, (1..8).map(|x| src.translate(0, x))),
        to_moves(board, src, (1..8).map(|x| src.translate(-x, 0))),
        to_moves(board, src, (1..8).map(|x| src.translate(0, -x))),
    ]
    .concat()
}

fn queen(board: &Board, src: Square) -> Vec<Move> {
    let mut b = bishop(board, src);
    b.append(&mut rook(board, src));
    b
}

fn pawn(board: &Board, src: Square) -> Vec<Move> {
    let (init_rank, rank_multiple) = board.turn.pawn_info();
    let to_move = |&sqr| {
        let sqr = sqr?;
        if board.get_piece(sqr).is_none() {
            Some(Move::Move(false, sqr, src))
        } else {
            None
        }
    };
    let mut moves: Vec<Move> = if src.rank == init_rank {
        [
            src.translate(0, rank_multiple),
            src.translate(0, 2 * rank_multiple),
        ]
        .iter()
        .map_while(to_move)
        .collect()
    } else {
        [src.translate(0, rank_multiple)]
            .iter()
            .map_while(to_move)
            .collect()
    };
    let mut captures = [-1, 1]
        .iter()
        .filter_map(|&dir| {
            let sqr = src.translate(dir, rank_multiple)?;
            match board.get_piece(sqr) {
                Some(piece) => {
                    if piece.owner != board.turn {
                        Some(Move::Move(false, sqr, src))
                    } else {
                        None
                    }
                }
                None => None,
            }
        })
        .collect();
    if let Some(en_passant) = board.en_passant {
        if let Some(true) = en_passant.translate(1, 0).map(|x| x == src) {
            moves.push(Move::EnPassent(src));
        } else if let Some(true) = en_passant.translate(-1, 0).map(|x| x == src) {
            moves.push(Move::EnPassent(src));
        }
    }
    moves.append(&mut captures);
    moves
}

fn king(board: &Board, src: Square) -> Vec<Move> {
    let moves = [
        src.translate(1, 0),
        src.translate(-1, 0),
        src.translate(1, 1),
        src.translate(1, -1),
        src.translate(0, -1),
        src.translate(0, 1),
        src.translate(-1, 1),
        src.translate(-1, -1),
    ];
    let mut moves: Vec<Move> = moves
        .iter()
        .filter_map(|&sqr| {
            let sqr = sqr?;
            let piece = board.get_piece(sqr);
            match piece {
                Some(piece) => {
                    if piece.owner == board.turn {
                        None
                    } else {
                        Some(Move::Move(false, sqr, src))
                    }
                }
                None => Some(Move::Move(true, sqr, src)),
            }
        })
        .collect();
    let (queen_side, king_side) = match board.turn {
        board::Player::White => (
            board.castling_rights.white_queen,
            board.castling_rights.white_king,
        ),
        board::Player::Black => (
            board.castling_rights.black_queen,
            board.castling_rights.black_king,
        ),
    };
    if king_side
        && board.get_piece(src.translate(1, 0).unwrap()).is_none()
        && board.get_piece(src.translate(2, 0).unwrap()).is_none()
    {
        moves.push(Move::Castle(true))
    }
    if queen_side
        && board.get_piece(src.translate(-1, 0).unwrap()).is_none()
        && board.get_piece(src.translate(-2, 0).unwrap()).is_none()
        && board.get_piece(src.translate(-3, 0).unwrap()).is_none()
    {
        moves.push(Move::Castle(false))
    }
    moves
}

pub fn get_move(board: &Board, src: Square) -> Option<Vec<Move>> {
    match board.get_piece(src).as_ref()?.kind {
        board::PieceKind::Knight => Some(knight(board, src)),
        board::PieceKind::Bishop => Some(bishop(board, src)),
        board::PieceKind::Rook => Some(rook(board, src)),
        board::PieceKind::Queen => Some(queen(board, src)),
        board::PieceKind::Pawn => Some(pawn(board, src)),
        board::PieceKind::King => Some(king(board, src)),
    }
}
