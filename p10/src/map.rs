use crate::pipe::{Tile, Direction, LoopTile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: usize,
    pub y: usize
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self { Self { x, y }}
}

pub struct Map {
    data: Vec<Vec<Tile>>,
    start: Coords
}

impl Map {
    pub fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let data: Vec<Vec<_>> = lines.map(|line|
            line.chars().map(Tile::from_char).collect())
            .collect();
        let start = data.iter().enumerate().filter_map(|(y, row)|
            row.iter().enumerate().find(|(_, el)| **el == Tile::Start).map(|(x, _)| Coords::new(x, y)))
            .nth(0).unwrap();
        Self { data, start }
    }

    pub fn height(&self) -> usize { self.data.len() }
    pub fn width(&self) -> usize { if self.height() > 0 { self.data[0].len() } else { 0 }}

    pub fn move_coords(&self, coords: Coords, dir: Direction) -> Option<Coords> {
        match dir {
            Direction::North => if coords.y == 0 { None } else { Some(Coords::new(coords.x, coords.y - 1)) },
            Direction::East => if coords.x + 1 >= self.width() { None } else { Some(Coords::new(coords.x + 1, coords.y)) },
            Direction::South => if coords.y + 1 >= self.height() { None } else { Some(Coords::new(coords.x, coords.y + 1)) },
            Direction::West => if coords.x == 0 { None } else { Some(Coords::new(coords.x - 1, coords.y)) },
        }
    }

    pub fn index(&self, coords: &Coords) -> &Tile { &self.data[coords.y][coords.x] }

    pub fn start_directions(&self) -> [Direction; 2] {
        let start = self.index(&self.start);
        let mut neighbors: Vec<_> = Direction::all().iter()
            .filter_map(|dir| self.move_coords(self.start, *dir)
                .and_then(|to_coords| if start.connect(self.index(&to_coords), *dir) { Some(*dir) } else { None })
            ).collect();
        if neighbors.len() != 2 { panic!("Did not find two neighbors of start"); };
        [neighbors[0], neighbors[1]]
    }

    pub fn half_distance(&self) -> usize {
        let mut current = self.start_directions().map(|dir| (self.start, dir));
        let mut iteration = 0usize;
        loop {
            let last_visited_0 = current[0].0;
            current.iter_mut().for_each(|el| {
                let coords = self.move_coords(el.0, el.1).unwrap();
                let dir = self.index(&coords).follow(el.1.reverse());
                *el = (coords, dir);
            });
            iteration += 1;
            if current[0].0 == current[1].0 || current[1].0 == last_visited_0 {
                break iteration;
            }
        }
    }

    pub fn to_loop_map(&self) -> LoopMap {
        let mut loop_data: Vec<Vec<_>> = self.data.iter().map(|row| row.iter().map(|_| LoopTile::NotPipe).collect()).collect();
        let start_dirs = self.start_directions();
        let mut at = self.start;
        let mut to_next = start_dirs[0];
        loop_data[at.y][at.x] = LoopTile::Pipe(start_dirs[0], start_dirs[1]);
        at = self.move_coords(at, to_next).unwrap();
        while at != self.start {
            to_next = self.index(&at).follow(to_next.reverse());
            loop_data[at.y][at.x] = match self.index(&at) {
                Tile::Pipe(d1, d2) => LoopTile::Pipe(*d1, *d2),
                Tile::Start => panic!(),
                Tile::Ground => panic!(),
            };
            at = self.move_coords(at, to_next).unwrap();
        }
        LoopMap { data: loop_data, start: self.start }
    }
}

fn count_row(row: &Vec<LoopTile>) -> usize {
    row.iter().fold((false, 0usize, None), |(inside, count, normal), tile| match tile {
        LoopTile::Pipe(d1, d2) => {
            match normal {
                Some(normal_dir) => if *d1 == Direction::East || *d2 == Direction::East {
                    (inside, count, normal)
                }
                else {
                    if *d1 == normal_dir || *d2 == normal_dir {
                       (inside, count, None)
                    }
                    else {
                        (!inside, count, None)
                    }
                },
                None => if *d1 == Direction::East {
                    (inside, count, Some(*d2))
                }
                else if *d2 == Direction::East {
                    (inside, count, Some(*d1))
                }
                else {
                    (!inside, count, None)
                },
            }
        },
        LoopTile::NotPipe => (inside, count + if inside { 1 } else { 0 }, None),
    }).1
}

pub struct LoopMap {
    data: Vec<Vec<LoopTile>>,
    start: Coords
}

impl LoopMap {
    pub fn count_inside(&self) -> usize {
        self.data.iter().map(count_row).sum()
    }
}