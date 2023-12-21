use crate::{vector2::Vector2, direction::Direction, dijkstra::dijkstra};

pub struct CityMap {
    blocks: Vec<Vec<u32>>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CityMapNode {
    Start(Vector2<usize>),
    End(Vector2<usize>),
    Node{
        index: Vector2<usize>,
        dir: Direction,
        count: u8
    }
}

impl CityMapNode {
    fn pos(&self) -> &Vector2<usize> {
        match self {
            CityMapNode::Start(pos) => pos,
            CityMapNode::End(pos) => pos,
            CityMapNode::Node { index, dir, count } => index
        }
    }

    fn node_or_end(index: Vector2<usize>, dir: Direction, count: u8, end: &Vector2<usize>) -> Self {
        if &index == end {
            CityMapNode::End(index)
        }
        else {
            CityMapNode::Node { index, dir, count }
        }
    }
}

impl CityMap {
    pub fn from_lines(lines: &mut impl Iterator<Item=String>) -> Self {
        Self { blocks: lines.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect() }
    }

    fn block(&self, idx: &Vector2<usize>) -> u32 {
        self.blocks[idx.y][idx.x]
    }

    fn height(&self) -> usize { self.blocks.len() }
    fn width(&self) -> usize { self.blocks.first().map(|row| row.len()).unwrap_or(0) }

    fn move_idx(&self, p: Vector2<usize>, d: Direction) -> Option<Vector2<usize>> {
        let pi = Vector2::new(p.x as i64, p.y as i64);
        let v = d.moved(&pi);
        if v.y < self.height() as i64 && v.x < self.width() as i64 {
            v.try_into().ok()
        }
        else {
            None
        }
    }

    pub fn navigate(&self, start: Vector2<usize>, end: Vector2<usize>) -> Option<u32> {
        dijkstra(
            CityMapNode::Start(start),
            &CityMapNode::End(end),
            |at| match at {
                CityMapNode::Start(idx) => Direction::all().filter_map(|dir| self.move_idx(*idx, dir)
                    .map(|p| CityMapNode::node_or_end(p, dir, 1, &end)))
                    .collect::<Vec<_>>().into_iter(),
                CityMapNode::End(_idx) => vec![].into_iter(),
                CityMapNode::Node { index, dir: from_dir, count } => {
                    let mut dirs = vec![from_dir.ccw(), from_dir.cw()];
                    if count < &3 { dirs.push(*from_dir) }
                    dirs.into_iter().filter_map(|dir| self.move_idx(*index, dir)
                    .map(|p| CityMapNode::node_or_end(p, dir, if &dir == from_dir { count + 1 } else { 1 }, &end)))
                    .collect::<Vec<_>>().into_iter()
                },
            },
            |a, b| {
                self.block(b.pos())
            })
    }

    pub fn navigate_p1(&self) -> u32 {
        self.navigate(Vector2::new(0, 0), Vector2::new(self.width() - 1, self.height() - 1)).unwrap()
    }
}