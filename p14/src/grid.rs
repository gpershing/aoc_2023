use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}};

use crate::tile::Tile;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    data: Vec<Vec<Option<Tile>>>
}

impl Grid {
    pub fn from_lines(lines: &mut impl Iterator<Item=String>) -> Self {
        Self { data: lines.map(|line| line.chars().map(Tile::from_char).collect()).collect() }
    }

    pub fn height(&self) -> usize { self.data.len() }
    pub fn width(&self) -> usize { self.data[0].len() }

    pub fn slide_and_rotate(&self) -> Self {
        let width = self.width();
        let height = self.height();
        let mut buf: Vec<Vec<Option<Tile>>> = (0..width).map(|_| (0..height).map(|_| None).collect()).collect();
        let mut hit: Vec<_> = (0..width).map(|_| 0u32).collect();
        for y in 0..height {
            for x in 0..width {
                match self.data[y][x] {
                    Some(Tile::Circle) => {
                        buf[x][height - 1 - hit[x] as usize] = Some(Tile::Circle);
                        hit[x] += 1;
                    },
                    Some(Tile::Square) => {
                        buf[x][height - 1 - y] = Some(Tile::Square);
                        hit[x] = y as u32 + 1;
                    }
                    None => ()
                }
            }
        }
        Self { data: buf }
    }

    pub fn as_lines(&self) -> String {
        self.data.iter().fold(String::new(), |s, l| { s + &l.iter().map(|t| match t {
            Some(Tile::Circle) => 'O',
            Some(Tile::Square) => '#',
            None => '.'
        }).collect::<String>() + "\n" })
    }

    pub fn weight(&self) -> usize {
        let mut sum = 0;
        let height = self.height();
        for y in 0..height {
            for x in 0..self.width() {
                match self.data[y][x] {
                    Some(Tile::Circle) => {
                        sum += height - y;
                    }
                    _ => ()
                }
            }
        }
        sum
    }

    pub fn hash_u64(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}