use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn from(text: &[u8]) -> Option<Position> {
        if text.len() != 2 {
            return None;
        }

        let x = (text[0] as i32) - ('a' as i32);
        let y = (text[1] as i32) - ('1' as i32);

        if x < 0 || x >= 8 || y < 0 || y >= 8 {
            return None;
        }

        Some(Position { x, y })
    }

    pub fn side(&self, idx: i32, dist: i32) -> Position {
        if dist <= 0 {
            panic!("Invalid distance '{}'", dist);
        }

        match idx {
            0 => Position::new(self.x, self.y + dist),
            1 => Position::new(self.x + dist, self.y),
            2 => Position::new(self.x, self.y - dist),
            3 => Position::new(self.x - dist, self.y),
            _ => panic!("Invalid index '{}'", idx),
        }
    }

    pub fn corner(&self, idx: i32, dist: i32) -> Position {
        if dist <= 0 {
            panic!("Invalid distance '{}'", dist);
        }

        match idx {
            0 => Position::new(self.x + dist, self.y + dist),
            1 => Position::new(self.x + dist, self.y - dist),
            2 => Position::new(self.x - dist, self.y + dist),
            3 => Position::new(self.x - dist, self.y - dist),
            _ => panic!("Invalid index '{}'", idx),
        }
    }

    pub fn is_valid(&self) -> bool {
        !(self.x < 0 || self.x >= 8 || self.y < 0 || self.y >= 8)
    }
}
