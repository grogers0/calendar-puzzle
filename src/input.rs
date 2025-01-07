use crate::piece::Piece;

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
}
