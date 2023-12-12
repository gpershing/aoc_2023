use std::collections::HashMap;

pub struct Destinations {
    pub left: String,
    pub right: String
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left, Right
}

impl Direction {
    pub fn from_char(c: char) -> Option<Direction> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None
        }
    }

    pub fn from_str(s: &str) -> Vec<Direction> {
        s.chars().filter_map(Direction::from_char).collect()
    }
}

pub trait Navigable {
    fn go(&self, at: &String, dir: Direction) -> String;
}

fn parse_line(line: String) -> (String, Destinations) {
    let parts: Vec<_> = line.split(" = ").collect();
    let source = parts[0].trim().to_string();
    let dest_part = parts[1].trim();
    let dests: Vec<_> = dest_part[1..dest_part.len()-1].split(", ").collect();
    (source, Destinations { left: dests[0].to_string(), right: dests[1].to_string() })
}

pub fn parse_map(lines: &mut impl Iterator<Item=String>) -> HashMap<String, Destinations> {
    let mut map = HashMap::new();
    map.extend(lines.map(parse_line));
    map
}

impl Navigable for HashMap<String, Destinations> {
    fn go(&self, at: &String, dir: Direction) -> String {
        match dir {
            Direction::Left => self[at].left.clone(),
            Direction::Right => self[at].right.clone(),
        }
    }
}