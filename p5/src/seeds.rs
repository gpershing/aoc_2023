#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeedRange {
    pub start: u64,
    pub length: u64
}

pub struct SeedRangeSplit {
    pub overlap: Option<SeedRange>,
    pub remaining: Vec<SeedRange>
}

impl SeedRange {
    pub fn new(start: u64, length: u64) -> Self { Self { start, length }}

    pub fn contains(&self, v: u64) -> bool {
        ((self.start)..(self.start + self.length)).contains(&v)
    }

    pub fn end(&self) -> u64 { self.start + self.length }

    pub fn overlap(&self, other: &SeedRange) -> Option<SeedRange> {
        if self.start + self.length <= other.start || other.start + other.length <= self.start {
            None
        }
        else {
            let start = self.start.max(other.start);
            let end = self.end().min(other.end());
            Some(SeedRange { start, length: end - start })
        }
    }

    pub fn split(self, other: &SeedRange) -> SeedRangeSplit {
        match self.overlap(other) {
            Some(overlap) => {
                let mut remaining = Vec::new();
                if self.start < overlap.start {
                    remaining.push(SeedRange::new(self.start, overlap.start - self.start));
                }
                if overlap.end() < self.end() {
                    remaining.push(SeedRange { start: overlap.end(), length: self.end() - overlap.end() });
                }
                SeedRangeSplit { overlap: Some(overlap), remaining }
            },
            None => SeedRangeSplit { overlap: None, remaining: vec![self] }
        }
    }
}

pub fn get_seeds(seed_line: String) -> Vec<u64> {
    if !seed_line.starts_with("seeds: ") {
        panic!("not the seed line");
    }
    seed_line["seeds: ".len()..].split_ascii_whitespace()
        .map(|part| part.parse().unwrap())
        .collect()
}

pub fn get_seed_ranges(seed_line: String) -> Vec<SeedRange> {
    if !seed_line.starts_with("seeds: ") {
        panic!("not the seed line");
    }
    let mut parts = seed_line["seeds: ".len()..].split_ascii_whitespace();
    let mut ranges = Vec::new();
    while let Some(start_str) = parts.next() {
        let start = start_str.parse().unwrap();
        let length = parts.next().unwrap().parse().unwrap();
        ranges.push(SeedRange { start, length });
    }
    ranges
}

#[cfg(test)]
mod test {
    use super::SeedRange;

    #[test]
    fn split_no_overlap() {
        let split = SeedRange::new(0, 10).split(&SeedRange::new(10, 10));
        assert_eq!(None, split.overlap);
        assert_eq!(1, split.remaining.len());
        assert_eq!(SeedRange::new(0, 10), split.remaining[0]);
    }

    #[test]
    fn split_one_remaining_above() {
        let split = SeedRange::new(0, 10).split(&SeedRange::new(0, 2));
        assert_eq!(Some(SeedRange { start: 0, length: 2 }), split.overlap);
        assert_eq!(1, split.remaining.len());
        assert_eq!(SeedRange::new(2, 8), split.remaining[0]);
    }

    #[test]
    fn split_one_remaining_below() {
        let split = SeedRange::new(0, 10).split(&SeedRange::new(8, 10));
        assert_eq!(Some(SeedRange { start: 8, length: 2 }), split.overlap);
        assert_eq!(1, split.remaining.len());
        assert_eq!(SeedRange::new(0, 8), split.remaining[0]);
    }

    #[test]
    fn split_two_remaining_below() {
        let split = SeedRange::new(0, 10).split(&SeedRange::new(2, 6));
        assert_eq!(Some(SeedRange { start: 2, length: 6 }), split.overlap);
        assert_eq!(2, split.remaining.len());
        assert_eq!(SeedRange::new(0, 2), split.remaining[0]);
        assert_eq!(SeedRange::new(8, 2), split.remaining[1]);
    }
}