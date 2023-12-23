use std::ops::{AddAssign, Add};

use crate::vector2::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn all() -> impl Iterator<Item=Self> {
        [Direction::North, Direction::East, Direction::South, Direction::West].into_iter()
    }

    pub fn ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn as_vector<T: From<i8>>(&self) -> Vector2<T> {
        match self {
            Direction::North => Vector2 { x: 0i8.into(), y: (-1i8).into() },
            Direction::East => Vector2 { x: 1i8.into(), y: 0i8.into() },
            Direction::South => Vector2 { x: 0i8.into(), y: 1i8.into() },
            Direction::West => Vector2 { x: (-1i8).into(), y: 0i8.into() },
        }
    }

    pub fn move_vector<T: From<i8> + AddAssign>(&self, v: &mut Vector2<T>) {
        *v += self.as_vector();
    }

    pub fn moved<T: From<i8> + Add<Output = T> + Copy>(&self, v: &Vector2<T>) -> Vector2<T> {
        *v + self.as_vector::<T>()
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}