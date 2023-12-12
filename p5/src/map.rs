use std::collections::VecDeque;

use crate::seeds::SeedRange;

pub struct Range {
    pub map_to_start: u64,
    pub map_from: SeedRange,
}

pub struct MapRangeResult {
    pub mapped: Option<SeedRange>,
    pub unmapped: Vec<SeedRange>
}

impl Range {
    pub fn from_line(line: String) -> Option<Self> {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        match parts.len() {
            0 => None,
            3.. => Some(Range { map_to_start: parts[0].parse().unwrap(), map_from: SeedRange { start: parts[1].parse().unwrap(), length: parts[2].parse().unwrap() } }),
            _ => panic!("Not enought components")
        }
    }

    pub fn map(&self, v: u64) -> Option<u64> {
        if self.map_from.contains(v) {
            Some((v - self.map_from.start) + self.map_to_start)
        }
        else {
            None
        }
    }

    pub fn map_range(&self, range: &SeedRange) -> MapRangeResult {
        let split = range.split(&self.map_from);
        MapRangeResult {
            mapped: split.overlap.map(|overlap| SeedRange { start: overlap.start - self.map_from.start + self.map_to_start, length: overlap.length }),
            unmapped: split.remaining
        }
    }
}

pub struct Map {
    pub name: String,
    pub ranges: Vec<Range>
}

impl Map {
    pub fn from_stream(iter: &mut impl Iterator<Item=String>) -> Option<Self> {
        let mut line = iter.next();
        while match &line {
            Some(l) => l.trim().len() == 0,
            None => false
        } {
            line = iter.next();
        }
        line.map(|title_line| {
            let name = title_line.split(" ").nth(0).unwrap().to_string();
            let ranges = iter.map_while(Range::from_line).collect();
            Map { name, ranges }
        })
    }

    pub fn map(&self, v: u64) -> u64 {
        self.ranges.iter().find_map(|range| range.map(v)).unwrap_or(v)
    }

    pub fn map_range(&self, input_range: &SeedRange) -> Vec<SeedRange> {
        let mut to_map = Vec::new();
        to_map.push(*input_range);
        let mut results = Vec::new();
        self.ranges.iter().for_each(|range| {
            let mut next_to_map = Vec::new();
            to_map.iter().for_each(|input| {
                let mut result = range.map_range(&input);
                if let Some(mapped) = result.mapped { results.push(mapped) }
                next_to_map.append(&mut result.unmapped);
            }); 
            to_map = next_to_map;
        });
        results.append(&mut to_map);
        results
    }
}

#[cfg(test)]
mod test {
    use crate::{map::Range, seeds::SeedRange};

    use super::Map;
    use test_case::test_case;

    #[test]
    fn test_range_parse_empty() {
        assert!(Range::from_line("".to_string()).is_none());
    }

    #[test]
    fn test_range_parse_space() {
        assert!(Range::from_line(" ".to_string()).is_none());
    }

    #[test]
    fn test_range_map_in_range() {
        let range = Range::from_line("1 11 2".to_string()).unwrap();
        assert_eq!(Some(1), range.map(11));
        assert_eq!(Some(2), range.map(12));
    }
    
    #[test]
    fn test_range_map_out_of_range() {
        let range = Range::from_line("1 11 2".to_string()).unwrap();
        assert_eq!(None, range.map(13));
    }

    #[test_case(SeedRange::new(101, 1), Some(SeedRange::new(1, 1)))]
    #[test_case(SeedRange::new(101, 100), Some(SeedRange::new(1, 49)))]
    #[test_case(SeedRange::new(99, 3), Some(SeedRange::new(0, 2)))]
    #[test_case(SeedRange::new(99, 100), Some(SeedRange::new(0, 50)))]
    #[test_case(SeedRange::new(97, 2), None)]
    #[test_case(SeedRange::new(150, 2), None)]
    fn test_range_map_range(input: SeedRange, result: Option<SeedRange>) {
        let range = Range::from_line("0 100 50".to_string()).unwrap();
        assert_eq!(result, range.map_range(&input).mapped);
    }

    #[test]
    fn test_map_parse_empty() {
        let map = Map::from_stream(&mut [" ".to_string()].into_iter());
        assert!(map.is_none());
    }

    #[test]
    fn test_map_parse_single_range() {
        let map = Map::from_stream(&mut [
            "a-to-b map:".to_string(),
            "1 11 2".to_string()
        ].into_iter()).unwrap();
        assert_eq!("a-to-b", &map.name);
        assert_eq!(1, map.ranges.len());
    }

    #[test]
    fn test_map_single_range_in_range() {
        let map = Map::from_stream(&mut [
            "a-to-b map:".to_string(),
            "1 11 2".to_string()
        ].into_iter()).unwrap();
        assert_eq!(1, map.map(11));
        assert_eq!(2, map.map(12));
    }

    #[test]
    fn test_map_single_range_out_of_range() {
        let map = Map::from_stream(&mut [
            "a-to-b map:".to_string(),
            "1 11 2".to_string()
        ].into_iter()).unwrap();
        assert_eq!(13, map.map(13));
    }

    #[test]
    fn test_map_multiple_ranges() {
        let map = Map::from_stream(&mut [
            "a-to-b map:".to_string(),
            "1 11 2".to_string(),
            "10 13 50".to_string()
        ].into_iter()).unwrap();
        assert_eq!(1, map.map(11));
        assert_eq!(2, map.map(12));
        assert_eq!(10, map.map(13));
        assert_eq!(40, map.map(43));
        assert_eq!(70, map.map(70));
    }

    #[test]
    fn test_map_single_range_range() {
        let map = Map::from_stream(&mut [
            "a-to-b map:".to_string(),
            "0 100 50".to_string()
        ].into_iter()).unwrap();
        let output = map.map_range(&SeedRange::new(99, 2));
        assert_eq!(2, output.len());
        assert_eq!(output[0], SeedRange::new(0, 1));
        assert_eq!(output[1], SeedRange::new(99, 1));
    }

    #[test]
    fn test_map_single_range_no_range() {
        let map = Map::from_stream(&mut [
            "a-to-b map:".to_string(),
            "0 100 50".to_string()
        ].into_iter()).unwrap();
        let output = map.map_range(&SeedRange::new(97, 2));
        assert_eq!(1, output.len());
        assert_eq!(SeedRange::new(97, 2), output[0]);
    }

    #[test]
    fn test_map_multiple_range_multiple_range_overlap() {
        let map = Map::from_stream(&mut [
            "a-to-b map:".to_string(),
            "0 100 50".to_string(),
            "0 150 50".to_string()
        ].into_iter()).unwrap();
        let output = map.map_range(&SeedRange::new(100, 75));
        assert_eq!(2, output.len());
        assert_eq!(SeedRange::new(0, 50), output[0]);
        assert_eq!(SeedRange::new(0, 25), output[1]);
    }

    #[test]
    fn test_map_real() {
        let map = Map::from_stream(&mut [
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string()
        ].into_iter()).unwrap();
        let output = map.map_range(&SeedRange::new(55, 13));
        assert_eq!(1, output.len());
        assert_eq!(SeedRange::new(57, 13), output[0]);
    }
}