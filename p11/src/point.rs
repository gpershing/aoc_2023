#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self { Self { x, y }}

    pub fn taxicab_dist(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

