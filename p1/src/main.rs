use std::{env, fs::File, io::{self, BufRead }, collections::HashMap};

fn get_lines(path: &String) -> impl Iterator<Item=String> {
    let file = File::open(path).unwrap_or_else(|_| panic!("Could not open the given file"));
    io::BufReader::new(file).lines()
        .map(|line| line.unwrap())
}

fn get_calibration_value_part1(line: String) -> u32 {
    let first = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
    let first_num = first.to_digit(10).unwrap();
    let last_num = last.to_digit(10).unwrap();
    first_num * 10 + last_num
}

fn get_written_digits() -> HashMap<String, u32> {
    let mut written_digits: HashMap<String, u32> = HashMap::new();
    written_digits.insert("one".to_string(), 1);
    written_digits.insert("two".to_string(), 2);
    written_digits.insert("three".to_string(), 3);
    written_digits.insert("four".to_string(), 4);
    written_digits.insert("five".to_string(), 5);
    written_digits.insert("six".to_string(), 6);
    written_digits.insert("seven".to_string(), 7);
    written_digits.insert("eight".to_string(), 8);
    written_digits.insert("nine".to_string(), 9);
    written_digits
}

fn get_digit(slice: &str, written_digits: &HashMap<String, u32>) -> Option<u32> {
    if let Some(digit) = slice.chars().nth(0).and_then(|c| c.to_digit(10)) {
        Some(digit)
    }
    else {
        written_digits.iter()
            .find(|(name, _)| slice.starts_with(name.as_str()))
            .map(|(_, digit)| *digit)
    }
}

fn get_calibration_value_part2(line: String) -> u32 {
    let written_digits = get_written_digits();
    let first = line.char_indices()
        .filter_map(|(index, c)| get_digit(&line[index..line.len()], &written_digits))
        .nth(0).unwrap();
    let last = line.char_indices().rev()
        .filter_map(|(index, c)| get_digit(&line[index..line.len()], &written_digits))
        .nth(0).unwrap();
    first * 10 + last
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Must provide the path of the file to open");
    }

    let part: u32 = args[1].parse().unwrap();
    let path = &args[2];
    let result: u32 = match part {
        1 => get_lines(path)
            .map(get_calibration_value_part1).sum(),
        2 => get_lines(path)
            .map(get_calibration_value_part2).sum(),
        _ => panic!()
    };

    println!("{result}");
}
