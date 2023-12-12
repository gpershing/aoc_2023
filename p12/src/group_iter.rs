use std::ops::Range;

use crate::row::Tile;

struct DataForIndex {
    min_count: usize,
    max_count: usize,
    min_remaining_count: usize,
    max_remaining_count: usize
}

pub struct GroupsIter {
    clue_count: usize,
    data: Vec<DataForIndex>,
    ranges: Vec<Range<usize>>
}

impl GroupsIter {
    fn first_ranges(data: &Vec<DataForIndex>) -> Vec<Range<usize>> {
        data.iter().map(|_| 0..0).collect()
        // let mut at = 0;
        // let mut remaining = clue_count;
        // let mut ranges = Vec::new();
        // for data in data {
        //     let max_count = (remaining - data.min_remaining_count).min(data.max_count);
        //     ranges.push(at..(at+max_count));
        //     at += max_count;
        //     remaining -= max_count;
        // }
        // ranges
    }

    pub fn new(tiles: &Vec<Vec<Tile>>, clues: &Vec<usize>) -> Self {
        let min_counts: Vec<_> = tiles.iter().map(|section| if section.contains(&Tile::Broken) { 1 } else { 0 }).collect();
        let required: usize = min_counts.iter().sum();
        let free = clues.len() - required;
        let max_counts: Vec<_> = tiles.iter().zip(min_counts.iter()).map(|(tile, min_count)| {
            (min_count + free).min((tile.len() + 1) / 2)
        }).collect();
        let min_remaining_counts: Vec<usize> = (0..min_counts.len()).map(|index| min_counts[index+1..].iter().sum()).collect();
        let max_remaining_counts: Vec<usize> = (0..max_counts.len()).map(|index| max_counts[index+1..].iter().sum()).collect();

        let data = min_counts.iter().zip(max_counts.iter()).zip(min_remaining_counts.iter().zip(max_remaining_counts.iter()))
            .map(|((&min_count, &max_count), (&min_remaining_count, &max_remaining_count))| DataForIndex { min_count, max_count, min_remaining_count, max_remaining_count })
            .collect();

        let ranges = Self::first_ranges(&data);
        let mut new = Self { clue_count: clues.len(), data, ranges };
        for index in 0..tiles.len() {
            new.maximize(index, tiles, clues);
        }
        new
    }

    fn reduce(&mut self, index: usize) -> bool {
        let entry = &self.ranges[index];
        let data = &self.data[index];
        if entry.len() > data.min_count {
            let remaining = self.clue_count - (self.ranges[index].end - 1);
            if remaining <= data.max_remaining_count {
                self.ranges[index].end -= 1;
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn maximize(&mut self, index: usize, tiles: &Vec<Vec<Tile>>, clues: &Vec<usize>) {
        let start = if index == 0 { 0 } else { self.ranges[index - 1].end };
        let remaining = self.clue_count - start;
        let mut available = tiles[index].len();
        let data = &self.data[index];
        let max_count = (remaining - data.min_remaining_count).min(data.max_count);
        let mut end = start + max_count;
        for to_add_index in start..(start+max_count) {
            if clues[to_add_index] <= available {
                available -= clues[to_add_index];
                if available > 0 {
                    available -= 1;
                }
            }
            else {
                end = to_add_index;
                break;
            }
        }
        self.ranges[index] = start..end
    }

    pub fn next(&mut self, tiles: &Vec<Vec<Tile>>, clues: &Vec<usize>) -> bool {
        if self.ranges.len() == 1 { return false; }

        let mut index = self.ranges.len() - 2;
        loop {
            while !self.reduce(index) {
                if index == 0 { return false; }
                index -= 1;
            }
            for max_index in (index+1)..self.ranges.len() {
                self.maximize(max_index, tiles, clues);
            }
            if self.ranges.last().unwrap().end == self.clue_count {
                break true;
            }
        }
    }

    pub fn current(&self) -> &Vec<Range<usize>> {
        &self.ranges
    }
}