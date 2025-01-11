mod board;
mod input;
mod piece;

fn main() {
    println!("{}\n", *input::BOARD);
    for p in input::PIECES.iter() {
        for p2 in p.permutations() {
            println!("{}\n--------", p2);
        }
        println!("====================");
    }
}
