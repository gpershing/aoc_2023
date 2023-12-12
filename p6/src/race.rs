#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Race {
    pub time: u64,
    pub distance: u64
}

impl Race {
    pub fn get_races(time_line: &str, distance_line: &str) -> Vec<Race> {
        let time_numbers = time_line.split(":").nth(1).unwrap()
            .split_ascii_whitespace()
            .map(|n_str| n_str.parse().unwrap());
        let distance_numbers = distance_line.split(":").nth(1).unwrap()
            .split_ascii_whitespace()
            .map(|n_str| n_str.parse().unwrap());
        time_numbers.zip(distance_numbers).map(|(time, distance)| Race { time, distance })
            .collect()
    }

    pub fn evaluate(&self) -> u64 {
        let b = -(self.time as f64);
        let c = self.distance as f64;
        let disc = (b*b - c*4.).sqrt();
        let min = (-b - disc) * 0.5;
        let max = (-b + disc) * 0.5;
        let diff = ((max-1.).ceil() - (min+1.).floor()) as u64 + 1;
        println!("{min} {max} {diff}");
        diff
    }
}

#[cfg(test)]
mod test {
    use super::Race;

    #[test]
    fn get_races_empty() {
        let races = Race::get_races("Time:    ", "Distance:     ");
        assert_eq!(0, races.len());
    }

    #[test]
    fn get_single_race() {
        let races = Race::get_races("Time:      1 ", "Distance:      2");
        assert_eq!(1, races.len());
        assert_eq!(Race { time: 1, distance: 2 }, races[0]);
    }

    #[test]
    fn get_multiple_races() {
        let races = Race::get_races("Time:      1   3  5", "Distance:      2    4   6");
        assert_eq!(3, races.len());
        assert_eq!(Race { time: 1, distance: 2 }, races[0]);
        assert_eq!(Race { time: 3, distance: 4 }, races[1]);
        assert_eq!(Race { time: 5, distance: 6 }, races[2]);
    }
}