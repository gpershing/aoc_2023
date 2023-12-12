use std::ops::Range;

use crate::{group_iter::GroupsIter, cache::{Cache, MyKey}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Broken,
    Unknown
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' => Tile::Broken,
            '?' => Tile::Unknown,
            _ => panic!()
        }
    }
}

#[derive(Debug)]
pub struct Row {
    tiles: Vec<Vec<Tile>>,
    clues: Vec<usize>
}

impl Row {
    fn from_parts(tile_part: &str, clue_part: &str) -> Self {
        let tiles: Vec<Vec<_>> = tile_part.replace(".", " ").split_ascii_whitespace()
            .map(|part| part.chars().map(Tile::from_char).collect()).collect();
        let clues = clue_part.split(',').map(|s| s.parse().unwrap()).collect();
        Self { tiles, clues }
    }

    pub fn from_line(l: String) -> Self {
        let parts: Vec<_> = l.split_ascii_whitespace().collect();
        Self::from_parts(parts[0], parts[1])
    }

    pub fn from_line_expanded(l: String, times: usize) -> Self {
        let parts: Vec<_> = l.split_ascii_whitespace().collect();
        let tile_part = std::iter::repeat(parts[0]).take(times).fold(None, |acc, next| Some(match acc {
            Some(s) => s + "?" + next,
            None => next.to_string()
        })).unwrap();
        let clue_part = std::iter::repeat(parts[1]).take(times).fold(None, |acc, next| Some(match acc {
            Some(s) => s + "," + next,
            None => next.to_string()
        })).unwrap();
        Self::from_parts(&tile_part, &clue_part)
    }

    fn min_required(clues: &[usize]) -> usize {
        if clues.len() == 0 { 0 } else { clues.iter().sum::<usize>() + clues.len() as usize - 1 }
    }

    fn count_possibilities_for_group_recursive(group: &[Tile], clues: &[usize]) -> usize {
        if clues.len() == 0 {
            if group.iter().any(|x| *x == Tile::Broken) {
                0
            }
            else {
                1
            }
        }
        else {
            let clue = clues[0];
            let min_req = Self::min_required(clues) as usize;
            if min_req > group.len() { return 0; }
            let max = group.len() - min_req;
            let mut result = 0;
            for start_index in 0..=max {
                let end_index = start_index + clue;
                if (start_index == 0 || group[start_index - 1] == Tile::Unknown) && (end_index >= group.len() || group[end_index] == Tile::Unknown) {
                    let next_start = if end_index + 1 < group.len() { end_index + 1 } else { end_index };
                    result += Self::count_possibilities_for_group_recursive(&group[next_start..], &clues[1..]);
                }
                if group[start_index] == Tile::Broken {
                    break;
                }
            }
            result
        }
    }

    fn count_possibilities_for_group(&self, index: usize, range: &Range<usize>, cache: &mut Cache) -> usize {
        match cache.get(&(self.tiles[index].as_slice(), &self.clues[range.clone()]) as &dyn MyKey) {
            Some(result) => *result,
            None => {
                let group = self.tiles[index].clone();
                let clues: Vec<_> = self.clues[range.clone()].iter().map(|x| *x).collect();
                let possibilities = Self::count_possibilities_for_group_recursive(&group, &clues);
                cache.insert((group, clues), possibilities);
                possibilities
            }
        }
    }

    fn count_possibilities_for(&self, group_ranges: &Vec<Range<usize>>, cache: &mut Cache) -> usize {
        group_ranges.iter().enumerate().rev().fold(1, |result, (index, range)| {
            if result == 0 {
                0
            }
            else {
                result * self.count_possibilities_for_group(index, range, cache)
            }
        })
    }

    pub fn possibilities(&self, cache: &mut Cache) -> usize {
        let mut result = 0;
        let mut group_iter = GroupsIter::new(&self.tiles, &self.clues);

        loop {
            let g = group_iter.current(); 
            result += self.count_possibilities_for(g, cache);
            if !group_iter.next(&self.tiles, &self.clues) {
                break;
            }
        }
        
        result
    }
} 

#[cfg(test)]
mod test {
    use test_case::test_case;
    use crate::cache::new_cache;

    use super::Row;

    #[test]
    fn vscode_help() {}

    #[test_case("???.### 1,1,3", 1)]
    #[test_case(".??..??...?##. 1,1,3", 4)]
    #[test_case("?###???????? 3,2,1", 10)]
    #[test_case("?.?.?.?.? 1", 5)]
    #[test_case("?.?.?.?.? 1,1", 10)]
    #[test_case("?#?.???.??? 3,2", 4)]
    #[test_case(".???.????.???#?? 3,1,5", 8)]
    #[test_case("????#??#?#??.????? 1,1,6,1,1,1", 10)] 
    #[test_case("???#?##???? 2,6", 2)]
    #[test_case("????##?#?#??. 1,8", 6)]
    #[test_case("??# 1", 1)]
    #[test_case(".?????# 2,1", 3)]
    #[test_case("????.??# 1,1", 5)]
    #[test_case(".#?#???????.????# 1,2,3,2,1", 6)]
    fn test_possibilities(line: &str, possibilities: usize) {
        assert_eq!(possibilities, Row::from_line(line.to_string()).possibilities(&mut new_cache()));
    }

    #[test_case("???.### 1,1,3", 1)]
    #[test_case("?.??#??.?.????.??.?? 1,4,1,1,1,2", 452659840; "hard")]
    fn test_expanded(line: &str, possibilities: usize) {
        assert_eq!(possibilities, Row::from_line_expanded(line.to_string(), 5).possibilities(&mut new_cache()));
    }
}