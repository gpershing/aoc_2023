use std::{ops::{AddAssign, Add}, collections::HashMap};

use crate::vector2::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn as_vector<T: From<i8>>(&self) -> Vector2<T> {
        match self {
            Direction::North => Vector2 { x: 0i8.into(), y: (-1i8).into() },
            Direction::East => Vector2 { x: 1i8.into(), y: 0i8.into() },
            Direction::South => Vector2 { x: 0i8.into(), y: 1i8.into() },
            Direction::West => Vector2 { x: (-1i8).into(), y: 0i8.into() },
        }
    }

    fn move_vector<T: From<i8> + AddAssign>(&self, v: &mut Vector2<T>) {
        *v += self.as_vector();
    }

    fn moved<T: From<i8> + Add<Output = T> + Copy>(&self, v: &Vector2<T>) -> Vector2<T> {
        *v + self.as_vector::<T>()
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DirectionSet {
    n: bool,
    e: bool,
    s: bool,
    w: bool
}

impl DirectionSet {
    fn new() -> Self { Self { n: false, e: false, s: false, w: false }}

    fn contains(&self, d: Direction) -> bool {
        match d {
            Direction::North => self.n,
            Direction::East => self.e,
            Direction::South => self.s,
            Direction::West => self.w,
        }
    }

    fn set(&mut self, d: Direction, v: bool) {
       *match d {
            Direction::North => &mut self.n,
            Direction::East => &mut self.e,
            Direction::South => &mut self.s,
            Direction::West => &mut self.w,
        } = v;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,      // .
    MirrorNE,   // /
    MirrorSE,   // \
    SplitterNS, // |
    SplitterEW, // -
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '/' => Tile::MirrorNE,
            '\\'=> Tile::MirrorSE,
            '|' => Tile::SplitterNS,
            '-' => Tile::SplitterEW,
            _ => Tile::Empty
        }
    }
}

pub enum ReflectResult {
    Dir(Direction),
    Split(Direction, Direction)
}

impl Tile {
    fn reflect(&self, dir: Direction) -> ReflectResult {
        match self {
            Tile::Empty => ReflectResult::Dir(dir),
            Tile::MirrorNE => ReflectResult::Dir(match dir {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            }),
            Tile::MirrorSE => ReflectResult::Dir(match dir {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
            }),
            Tile::SplitterNS => match dir {
                Direction::East | Direction::West => ReflectResult::Split(Direction::North, Direction::South),
                _ => ReflectResult::Dir(dir)
            },
            Tile::SplitterEW => match dir {
                Direction::South | Direction::North => ReflectResult::Split(Direction::East, Direction::West),
                _ => ReflectResult::Dir(dir)
            },
        }
    }
}

pub struct Puzzle {
    tiles: Vec<Vec<Tile>>
}

impl Puzzle {
    pub fn from_lines(l: &mut impl Iterator<Item=String>) -> Self {
        let tiles = l.map(|line| line.chars().map(Tile::from_char).collect()).collect();
        Self { tiles }
    }

    fn get_tile(&self, p: Vector2<usize>) -> Tile {
        self.tiles[p.y][p.x]
    }

    fn height(&self) -> usize { self.tiles.len() }
    fn width(&self) -> usize { self.tiles.first().map(|row| row.len()).unwrap_or(0) }

    fn move_bounded(&self, p: Vector2<usize>, d: Direction) -> Option<Vector2<usize>> {
        let pi = Vector2::new(p.x as i64, p.y as i64);
        let v = d.moved(&pi);
        if v.y < self.height() as i64 && v.x < self.width() as i64 {
            v.try_into().ok()
        }
        else {
            None
        }
    }

    fn reflect(&self, p: Vector2<usize>, d: Direction) -> ReflectResult {
        self.get_tile(p).reflect(d)
    }

    pub fn eval(&self) -> usize { self.eval_from(Vector2 { x: 0, y: 0 }, Direction::East) }

    pub fn eval_from(&self, start_p: Vector2<usize>, start_d: Direction) -> usize {
        let mut hit: HashMap<Vector2<usize>, DirectionSet> = HashMap::new();
        let mut to_run = Vec::new();
        to_run.push((start_p, start_d));

        while let Some((p, d)) = to_run.pop() {
            if let Some(d_set) = hit.get_mut(&p) {
                if d_set.contains(d) {
                    continue;
                }
                else {
                    d_set.set(d, true);
                }
            }
            else {
                let mut new_set = DirectionSet::new();
                new_set.set(d, true);
                hit.insert(p, new_set);
            }

            match self.reflect(p, d) {
                ReflectResult::Dir(next_d) => {
                    if let Some(next_p) = self.move_bounded(p, next_d) {
                        to_run.push((next_p, next_d));
                    }
                },
                ReflectResult::Split(next_d1, next_d2) => {
                    if let Some(next_p1) = self.move_bounded(p, next_d1) {
                        to_run.push((next_p1, next_d1));
                    }
                    if let Some(next_p2) = self.move_bounded(p, next_d2) {
                        to_run.push((next_p2, next_d2));
                    }
                },
            }
        }

        hit.len()
    }

    pub fn starting_options(&self) -> impl Iterator<Item=(Vector2<usize>, Direction)> {
        let w = self.width();
        let h = self.height();
        (0..w).map(|x| (Vector2::new(x, 0), Direction::South))
            .chain((0..w).map(move |x| (Vector2::new(x, h - 1), Direction::North)))
            .chain((0..h).map(|y| (Vector2::new(0, y), Direction::East)))
            .chain((0..h).map(move |y| (Vector2::new(w - 1, y), Direction::West)))
    }

    pub fn eval_max(&self) -> usize {
        self.starting_options().map(|(p, d)| self.eval_from(p, d)).max().unwrap()
    }
}