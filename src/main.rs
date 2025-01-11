mod board;
mod input;
mod piece;
mod pos;

use pos::Pos;

fn main() {
    let mut b = input::BOARD.clone();
    b.place(&input::PIECES[2], &Pos::new(0, 0));
    println!("{}\n", b);
}
