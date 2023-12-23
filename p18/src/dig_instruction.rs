use crate::direction::Direction;

pub struct DigInstruction {
    pub direction: Direction,
    pub count: u64,
}

impl DigInstruction {
    pub fn from_line(line: &str) -> DigInstruction {
        let parts: Vec<_> = line.split(" ").collect();
        let direction = match parts[0] {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            _ => panic!()
        };
        let count = parts[1].parse().unwrap();
        Self { direction, count }
    }

    pub fn from_hex(line: &str) -> DigInstruction {
        let hex_part = line.split(" ").nth(2).unwrap();
        let hex_part = &hex_part[2..hex_part.len()-1];
        let hex_digits: Vec<_> = hex_part.chars().map(|c| c.to_digit(16).unwrap() as u64).collect();

        let mut count = 0;
        for digit in &hex_digits[0..5] {
            count = (count * 16) + digit;
        }
        let direction = match hex_digits[5] {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => panic!()
        };
        Self { direction, count }
    }
}