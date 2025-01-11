use crate::piece::Piece;
use crate::board::Board;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PIECES: Vec<Piece> = vec![
        Piece::parse("##\n##"),
        Piece::parse("# \n##\n# "),
        Piece::parse("## \n ##"),
        Piece::parse(" ##\n###"),
        Piece::parse("#  \n###"),
        Piece::parse("#   \n####"),
        Piece::parse(" # \n###\n # "),
        Piece::parse("#  \n###\n  #"),
        Piece::parse("# #\n###"),
    ];

    pub static ref BOARD: Board = Board::parse(
        "      #\n      #\n       \n       \n       \n       \n##   ##");
}
