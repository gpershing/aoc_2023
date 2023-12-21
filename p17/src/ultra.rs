use crate::{vector2::Vector2, direction::Direction, dijkstra::dijkstra};

pub struct UltraCityMap {
    blocks: Vec<Vec<u32>>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UltraCityMapNode {
    Start(Vector2<usize>),
    End(Vector2<usize>),
    Node{
        index: Vector2<usize>,
        dir: Direction,
        count: u8
    }
}

impl UltraCityMapNode {
    fn pos(&self) -> &Vector2<usize> {
        match self {
            UltraCityMapNode::Start(pos) => pos,
            UltraCityMapNode::End(pos) => pos,
            UltraCityMapNode::Node { index, dir: _dir, count: _count } => index
        }
    }

    fn node_or_end(index: Vector2<usize>, dir: Direction, count: u8, end: &Vector2<usize>) -> Self {
        if &index == end && count >= 4 {
            UltraCityMapNode::End(index)
        }
        else {
            UltraCityMapNode::Node { index, dir, count }
        }
    }
}

impl UltraCityMap {
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
            UltraCityMapNode::Start(start),
            &UltraCityMapNode::End(end),
            |at| match at {
                UltraCityMapNode::Start(idx) => Direction::all().filter_map(|dir| self.move_idx(*idx, dir)
                    .map(|p| UltraCityMapNode::node_or_end(p, dir, 1, &end)))
                    .collect::<Vec<_>>().into_iter(),
                UltraCityMapNode::End(_idx) => vec![].into_iter(),
                UltraCityMapNode::Node { index, dir: from_dir, count } => {
                    let mut dirs = Vec::new();
                    if count < &4 {
                        dirs.push(*from_dir);
                    }
                    else {
                        dirs.push(from_dir.ccw());
                        dirs.push(from_dir.cw());
                        if count < &10 { dirs.push(*from_dir) }
                    }
                    dirs.into_iter().filter_map(|dir| self.move_idx(*index, dir)
                    .map(|p| UltraCityMapNode::node_or_end(p, dir, if &dir == from_dir { count + 1 } else { 1 }, &end)))
                    .collect::<Vec<_>>().into_iter()
                },
            },
            |_a, b| {
                self.block(b.pos())
            })
    }

    pub fn navigate_p(&self) -> u32 {
        self.navigate(Vector2::new(0, 0), Vector2::new(self.width() - 1, self.height() - 1)).unwrap()
    }
}