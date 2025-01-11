use std::fmt;
use std::collections::HashSet;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Piece {
    width: usize,
    height: usize,
    grid: Vec<bool>,
}

impl Piece {
    pub fn parse(s: &str) -> Piece {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();
        let mut grid = vec![false; width * height];
        for (y, line) in s.lines().enumerate() {
            assert_eq!(line.chars().count(), width, "must be rectangular");
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        grid[y * width + x] = true;
                    },
                    '.' | ' ' => (),
                    _ => panic!("unknown char '{}' for piece", ch),
                }
            }
        }
        Piece { width, height, grid }
    }

    // Flip horizontally
    pub fn flipped(&self) -> Piece {
        let width = self.width;
        let height = self.height;
        let mut grid = vec![false; width * height];
        for y in 0..height {
            for x in 0..width {
                grid[y * width + x] =
                    self.grid[y * self.width + (width - 1 - x)];
            }
        }
        Piece { width, height, grid }
    }

    // Rotate 90 degrees clockwise
    pub fn rotated(&self) -> Piece {
        let width = self.height;
        let height = self.width;
        let mut grid = vec![false; width * height];
        for y in 0..height {
            for x in 0..width {
                grid[y * width + x] =
                    self.grid[(self.height - 1 - x) * self.width + y];
            }
        }
        Piece { width, height, grid }
    }

    pub fn permutations(&self) -> HashSet<Piece> {
        let mut ret = HashSet::<Piece>::new();
        let mut p = self.clone();
        ret.insert(p.clone());
        p = p.rotated();
        ret.insert(p.clone());
        p = p.rotated();
        ret.insert(p.clone());
        p = p.rotated();
        ret.insert(p.clone());
        p = self.flipped();
        ret.insert(p.clone());
        p = p.rotated();
        ret.insert(p.clone());
        p = p.rotated();
        ret.insert(p.clone());
        p = p.rotated();
        ret.insert(p.clone());
        ret
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            if y != 0 { write!(f, "\n")?; }
            for x in 0..self.width {
                let ch = if self.grid[y * self.width + x] {
                    '#'
                } else {
                    ' '
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
        let s = "###\n # ";
        let p = Piece::parse(s);
        assert_eq!(s, format!("{}", p));
    }

    #[test]
    fn test_flip() {
        let s = "# \n##\n #";
        let p = Piece::parse(s);
        let expected = " #\n##\n# ";
        assert_eq!(expected, format!("{}", p.flipped()));
        assert_eq!(p, p.flipped().flipped());
    }

    #[test]
    fn test_rotate() {
        let s = " # \n###";
        let p = Piece::parse(s);
        let expected = "# \n##\n# ";
        assert_eq!(expected, format!("{}", p.rotated()));
        assert_eq!(p, p.rotated().rotated().rotated().rotated());
    }
}
