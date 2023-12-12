use std::env;

use crate::{get_line_group::get_line_groups, get_lines::get_lines};

fn as_digit_unchecked(row: &Vec<u8>, index: usize) -> Option<char> {
    let char = row[index] as char;
    if char.is_ascii_digit() { Some(char) } else { None }
}

fn as_digit(row: &Vec<u8>, index: usize) -> Option<char> {
    if index < row.len() { as_digit_unchecked(row, index) } else { None }
}

fn expand_before(row: &Vec<u8>, index: usize) -> u32 {
    let mut number = String::new();
    let mut index = index;
    while let Some(char) = as_digit_unchecked(row, index) {
        number = char.to_string() + &number;
        if index > 0 {
            index -= 1;
        }
        else {
            break;
        }
    }
    number.parse().unwrap()
}

fn expand_after(row: &Vec<u8>, index: usize) -> u32 {
    let mut number = String::new();
    let mut index = index;
    while let Some(char) = as_digit(row, index) {
        number += &char.to_string();
        index += 1;
    }
    number.parse().unwrap()
}

fn expand_both(row: &Vec<u8>, index: usize) -> u32 {
    let mut index = index;
    let first_idx = loop {
        if !(row[index].is_ascii_digit()) {
            break index + 1;
        }
        else if index == 0 {
            break 0;
        }
        else {
            index -= 1;
        }
    };
    expand_after(row, first_idx)
}

fn find_numbers(row: &Vec<u8>, gear_index: usize) -> [Option<u32>; 2] {
    let before = gear_index >= 1 && (row[gear_index-1] as char).is_ascii_digit();
    let mid = (row[gear_index] as char).is_ascii_digit();
    let after = gear_index + 1 < row.len() && (row[gear_index+1] as char).is_ascii_digit();
    match (before, mid, after) {
        (true, true, true) => [Some(expand_both(row, gear_index)), None],
        (true, true, false) => [Some(expand_before(row, gear_index)), None],
        (true, false, true) => [Some(expand_before(row, gear_index - 1)), Some(expand_after(row, gear_index+1))],
        (true, false, false) => [Some(expand_before(row, gear_index - 1)), None],
        (false, true, true) => [Some(expand_after(row, gear_index)), None],
        (false, true, false) => [(row[gear_index] as char).to_string().parse().ok(), None],
        (false, false, true) => [Some(expand_after(row, gear_index + 1)), None],
        (false, false, false) => [None, None],
    }
}

fn evaluate_gear(group: &[Vec<u8>; 3], index: usize) -> u32 {
    let numbers: Vec<_> = group.iter().map(|row| find_numbers(row, index).into_iter())
        .flatten()
        .filter_map(std::convert::identity)
        .collect();

    if numbers.len() == 2 {
        numbers[0] * numbers[1]
    }
    else {
        0
    }
}

fn gear_ratio_count_for_line_group(group: [String; 3]) -> u32 {
    let gear = '*' as u8;
    let byte_groups = group.map(String::into_bytes);

    byte_groups[1].iter().enumerate()
        .filter(|(_, el)| **el == gear)
        .map(|(index, _)| evaluate_gear(&byte_groups, index))
        .sum()
}

pub fn part2_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Must provide the path of the file to open");
    }

    let path = &args[2];
    let result: u32 = get_line_groups(get_lines(path), '.')
        .map(gear_ratio_count_for_line_group)
        .sum();

    println!("{result}");
}

#[cfg(test)]
mod test {
    use crate::part2::gear_ratio_count_for_line_group;

    #[test]
    fn gear_ratio_count_no_number() {
        let group = [
            "1...1".to_string(),
            "1.*.1".to_string(),
            "1...1".to_string()
        ];
        assert_eq!(0, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_one_number() {
        let group = [
            ".12..".to_string(),
            "..*..".to_string(),
            ".....".to_string()
        ];
        assert_eq!(0, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_three_numbers() {
        let group = [
            "12...".to_string(),
            "..*3.".to_string(),
            "..45.".to_string()
        ];
        assert_eq!(0, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_four_numbers() {
        let group = [
            "12...".to_string(),
            ".2*3.".to_string(),
            "..45.".to_string()
        ];
        assert_eq!(0, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_gear_at_start() {
        let group = [
            "12...".to_string(),
            "*3...".to_string(),
            ".....".to_string()
        ];
        assert_eq!(36, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_at_mid() {
        let group = [
            "...3.".to_string(),
            "..*..".to_string(),
            "12...".to_string()
        ];
        assert_eq!(36, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_at_end() {
        let group = [
            "...12".to_string(),
            "....*".to_string(),
            "...3.".to_string()
        ];
        assert_eq!(36, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_two_adjacent_short() {
        let group = [
            ".....".to_string(),
            ".2*3.".to_string(),
            ".....".to_string()
        ];
        assert_eq!(6, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_two_adjacent() {
        let group = [
            ".....".to_string(),
            "12*30".to_string(),
            ".....".to_string()
        ];
        assert_eq!(360, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_long_above_and_below() {
        let group = [
            ".1000".to_string(),
            "..*..".to_string(),
            "10000".to_string()
        ];
        assert_eq!(10000000, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_two_gears() {
        let group = [
            "...12".to_string(),
            "..*.*".to_string(),
            "...3.".to_string()
        ];
        assert_eq!(72, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_test1() {
        let group = [
            "..123.".to_string(),
            "....*.".to_string(),
            ".....1".to_string()
        ];
        assert_eq!(123, gear_ratio_count_for_line_group(group));
    }

    #[test]
    fn gear_ratio_count_test2() {
        let group = [
            "123456".to_string(),
            "...*..".to_string(),
            "...1..".to_string()
        ];
        assert_eq!(123456, gear_ratio_count_for_line_group(group));
    }
    
}