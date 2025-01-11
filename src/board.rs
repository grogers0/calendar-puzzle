use std::fmt;

use crate::piece::Piece;
use crate::pos::Pos;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Wall,
    Piece,
    Goal,
}

impl Cell {
    fn is_empty(&self) -> bool {
        *self == Cell::Empty
    }
}


#[derive(Clone)]
pub struct Board {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}

impl Board {
    pub fn parse(s: &str) -> Board {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();
        let mut grid = Vec::with_capacity(width * height);
        for line in s.lines() {
            assert_eq!(line.chars().count(), width, "Must be rectangular");
            for ch in line.chars() {
                let cell = match ch {
                    ' ' => Cell::Empty,
                    '.' => Cell::Piece,
                    '#' => Cell::Wall,
                    '*' => Cell::Goal,
                    _ => panic!("unexpected character '{}'", ch),
                };
                grid.push(cell);
            }
        }
        Board { width, height, grid }
    }

    pub fn is_filled(&self) -> bool {
        for cell in &self.grid {
            if cell.is_empty() { return false; }
        }
        true
    }

    pub fn can_place(&self, piece: &Piece, pos: &Pos) -> bool {
        if pos.x + piece.width > self.width { return false; }
        if pos.y + piece.height > self.height { return false; }
        for y in 0..piece.height {
            for x in 0..piece.width {
                if piece.grid[y * piece.width + x] {
                    let idx = (y + pos.y) * self.width + (x + pos.x);
                    if !self.grid[idx].is_empty() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn place(&mut self, piece: &Piece, pos: &Pos) {
        debug_assert!(self.can_place(piece, pos));
        for y in 0..piece.height {
            for x in 0..piece.width {
                if piece.grid[y * piece.width + x] {
                    let idx = (y + pos.y) * self.width + (x + pos.x);
                    self.grid[idx] = Cell::Piece;
                }
            }
        }
    }

    // Caller must ensure the piece was meant to be placed there
    pub fn unplace(&mut self, piece: &Piece, pos: &Pos) {
        for y in 0..piece.height {
            for x in 0..piece.width {
                if piece.grid[y * piece.width + x] {
                    let idx = (y + pos.y) * self.width + (x + pos.x);
                    debug_assert_eq!(self.grid[idx], Cell::Piece);
                    self.grid[idx] = Cell::Empty;
                }
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            if y != 0 { write!(f, "\n")?; }
            for x in 0..self.width {
                let ch = match self.grid[y * self.width + x] {
                    Cell::Empty => ' ',
                    Cell::Wall => '#',
                    Cell::Piece => '.',
                    Cell::Goal => '*',
                };
                write!(f, "{}", ch)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_display() {
        let s = "  #..\n   ..\n# *  \n   .*";
        let b = Board::parse(s);
        assert_eq!(s, format!("{}", b));
    }
}
