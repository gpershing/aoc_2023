#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Circle,
    Square    
}

impl Tile {
    pub fn from_char(c: char) -> Option<Tile> {
        match c {
            'O' => Some(Tile::Circle),
            '#' => Some(Tile::Square),
            _ => None
        }
    }
}