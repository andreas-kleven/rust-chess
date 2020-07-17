use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub num: i32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num;
    }
}

impl Player {
    pub fn is_white(&self) -> bool {
        self.num == 1
    }

    pub fn is_black(&self) -> bool {
        self.num == 2
    }

    pub fn is_none(&self) -> bool {
        !self.is_white() && !self.is_black()
    }
}
