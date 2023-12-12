#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn all() -> [Self; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Pipe(Direction, Direction),
    Start,
    Ground,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '|' => Tile::Pipe(Direction::North, Direction::South),
            '-' => Tile::Pipe(Direction::East, Direction::West),
            'L' => Tile::Pipe(Direction::North, Direction::East),
            'J' => Tile::Pipe(Direction::North, Direction::West),
            '7' => Tile::Pipe(Direction::South, Direction::West),
            'F' => Tile::Pipe(Direction::East, Direction::South),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!()
        }
    }

    pub fn connect(&self, other: &Self, direction: Direction) -> bool {
        match (self, other) {
            (Tile::Ground, _) => false,
            (_, Tile::Ground) => false,
            (Tile::Start, Tile::Start) => true,
            (Tile::Pipe(a1, a2), Tile::Start) => *a1 == direction || *a2 == direction,
            (Tile::Start, Tile::Pipe(b1, b2)) => *b1 == direction.reverse() || *b2 == direction.reverse(),
            (Tile::Pipe(a1, a2), Tile::Pipe(b1, b2)) => 
                *a1 == direction || *a2 == direction && *b1 == direction.reverse() || *b2 == direction.reverse(),
        }
    }

    pub fn follow(&self, from_direction: Direction) -> Direction {
        match self {
            Tile::Pipe(a1, a2) => if *a1 == from_direction { *a2 } else { *a1 },
            Tile::Start => panic!("Cannot follow start"),
            Tile::Ground => panic!("Cannot follow ground"),
        }
    }
}

pub enum LoopTile {
    Pipe(Direction, Direction),
    NotPipe
}