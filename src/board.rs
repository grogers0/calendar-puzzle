use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Wall,
    Piece,
    Goal,
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
        for space in &self.grid {
            if let Cell::Empty = space { return false; }
        }
        true
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
                write!(f, "{}", ch);
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
