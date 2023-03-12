/// The kind of a chess peice
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// The 2 possible players
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black,
}

/// The piece itself
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub owner: Player,
}

#[derive(Clone, Copy)]
pub struct Square {
    pub rank: u8,
    pub file: u8,
}

impl Square {
    pub fn new(file: u8, rank: u8) -> Option<Square> {
        if file < 8 && rank < 8 {
            Some(Square { file, rank })
        } else {
            None
        }
    }
    pub fn translate(self, file: i32, rank: i32) -> Option<Square> {
        let file = self.file as i32 - file;
        let rank = self.rank as i32 - rank;
        if file < 8 && rank < 8 {
            Some(Square {
                rank: rank.try_into().ok()?,
                file: file.try_into().ok()?,
            })
        } else {
            None
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        write!(f, "{}{}", files[self.file as usize], self.rank + 1)
    }
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(self, f)
    }
}

/// The board representation
#[derive(Debug)]
pub struct Board {
    /// The piece placement
    pub positions: [Option<Piece>; 64],
    /// The player that should play in the current move
    pub turn: Player,
    /// (white queen castle, white king castle, black queen castle, black king castle)
    pub castling_rights: (bool, bool, bool, bool),
    /// The en passent target square
    pub en_passent: Option<Square>,
    /// The number of moves that does not envolve a capture/pawn push
    pub reversible_moves: u16,
    /// The number of moves
    pub full_moves: u16,
}

pub enum BoardStringKind {
    Fen,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("fen error: {0}")]
    FenError(#[from] crate::fen::Error),
}

impl Board {
    pub fn new(s: &str, kind: &BoardStringKind) -> Result<Self, Error> {
        Ok(match kind {
            BoardStringKind::Fen => crate::fen::parse(s)?,
        })
    }
    pub fn get_piece<'a>(&'a self, s: Square) -> &'a Option<Piece> {
        &self.positions[(s.rank * 8 + s.file) as usize]
    }
}
