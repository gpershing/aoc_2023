#[derive(Debug,Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Ash,
    Rocks
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Ash,
            '#' => Tile::Rocks,
            _ => panic!()
        }
    }
}

pub struct Grid {
    rows: Vec<Vec<Tile>>
}

fn error_match<T>(lhs: &[T], rhs: &[T], max_diff: usize) -> Option<usize> where T: Eq {
    let mut diff = 0;
    for (l, r) in lhs.iter().zip(rhs.iter()) {
        if l != r {
            diff += 1;
            if diff > max_diff {
                return None
            }
        }
    }
    Some(diff)
}

impl Grid {
    pub fn from_lines(lines: &mut impl Iterator<Item=String>) -> Self {
        Self { rows: lines.map(|line| line.chars().map(Tile::from_char).collect()).collect() }
    }

    pub fn transposed(&self) -> Self {
        let cols = (0..self.rows[0].len()).map(|index| {
            self.rows.iter().map(|row| row[index]).collect()
        }).collect();
        Self { rows: cols }
    }

    fn row_matches(&self, match_index: usize) -> impl Iterator<Item=usize> + '_ {
        self.rows.iter().enumerate().filter_map(move |(index, row)| {
            if (match_index % 2) != (index % 2) // disallow mirror axes on rows
                && row == &self.rows[match_index] {
                Some(index)
            }
            else {
                None
            }
        })
    }

    fn row_almost_matches(&self, match_index: usize) -> impl Iterator<Item=(usize, usize)> + '_ {
        self.rows.iter().enumerate().filter_map(move |(index, row)| {
            if (match_index % 2) != (index % 2) {
                error_match(&row, &self.rows[match_index], 1).map(|diff| (index, diff))
            }
            else {
                None
            }
        })
    }

    fn is_mirror_match_edge(&self, index_a: usize, index_b: usize) -> Option<usize> {
        let min = index_a.min(index_b);
        let max = index_a.max(index_b);
        let midpoint = (min + max) / 2;
        if (min+1..=midpoint).all(|lower| {
            let upper = max - (lower - min);
            &self.rows[lower] == &self.rows[upper]
        }) {
            Some(midpoint + 1)
        }
        else {
            None
        }
    }

    fn is_almost_mirror_match_edge(&self, index_a: usize, index_b: usize) -> Option<usize> {
        let min = index_a.min(index_b);
        let max = index_a.max(index_b);
        let midpoint = (min + max) / 2;
        let mut diff_left = 1;
        let mut is_mirror = true;
        for lower in min+1..=midpoint {
            let upper = max - (lower - min);
            match error_match(&self.rows[lower], &self.rows[upper], diff_left) {
                Some(diff) => {
                    diff_left -= diff;
                },
                None => {
                    is_mirror = false;
                    break;
                }
            }
        }
        if is_mirror && diff_left == 0 {
            Some(midpoint + 1)
        }
        else {
            None
        }
    }

    fn mirrors_from(&self, edge_index: usize) -> Option<usize> {
        self.row_matches(edge_index)
            .filter_map(|index| self.is_mirror_match_edge(edge_index, index))
            .nth(0)
    }

    fn almost_mirrors_from(&self, edge_index: usize) -> Option<usize> {
        self.row_almost_matches(edge_index)
            .filter_map(|(index, diff)| if diff == 1 {
                self.is_mirror_match_edge(edge_index, index)
            }
            else {
                self.is_almost_mirror_match_edge(edge_index, index)
            })
            .nth(0)
    }

    pub fn mirror_eval(&self) -> Option<usize> {
        self.mirrors_from(0).or_else(|| self.mirrors_from(self.rows.len() - 1))
    }

    pub fn mirror_eval_with_transpose(&self) -> usize {
        self.mirror_eval().map(|x| x * 100).or_else(|| self.transposed().mirror_eval()).unwrap()
    }

    pub fn mirror_almost_eval(&self) -> Option<usize> {
        self.almost_mirrors_from(0).or_else(|| self.almost_mirrors_from(self.rows.len() - 1))
    }

    pub fn mirror_almost_eval_with_transpose(&self) -> usize {
        self.mirror_almost_eval().map(|x| x * 100).or_else(|| self.transposed().mirror_almost_eval()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use super::Grid;

    #[test]
    fn vscode_help() { }

    #[test_case("..#.#......#.
###.#.####.#.
##.##.#..#.##
##..#.####.#.
...###.##.###
###.##.##.##.
###..##..#...
###..##..##..
####.#.##.#.#", 8)]
    fn test_almost_mirror(s: &str, result: usize) {
        assert_eq!(result, Grid::from_lines(&mut s.lines().map(|l| l.to_string())).mirror_almost_eval_with_transpose())
    }
}