use std::env;

use crate::get_lines::get_lines;
use crate::get_line_group::get_line_groups;

fn is_symbol(char: u8) -> bool {
    let character = char as char;
    !character.is_ascii_digit() && character != '.'
}

fn is_counted_col(group: &[Vec<u8>; 3], index: usize) -> bool {
    is_symbol(group[0][index]) || is_symbol(group[1][index]) || is_symbol(group[2][index])
}

fn is_counted_above_and_below(group: &[Vec<u8>; 3], index: usize) -> bool {
    is_symbol(group[0][index]) || is_symbol(group[2][index])
}

fn count_for_line_group(group: [String; 3]) -> u32 {
    let byte_groups = group.map(String::into_bytes);
    let mut number = String::new();
    let mut counted = false;
    let mut sum = 0u32;
    for index in 0..byte_groups[1].len() {
        let character = byte_groups[1][index] as char;
        if character.is_ascii_digit() {
            if number.is_empty() && index > 0 {
                counted = is_counted_col(&byte_groups, index - 1);
            }
            number += &character.to_string();
            counted = counted || is_counted_above_and_below(&byte_groups, index);

            if index + 1 >= byte_groups[1].len() || !(byte_groups[1][index+1] as char).is_ascii_digit() {
                if counted || (index + 1 < byte_groups[1].len() && is_counted_col(&byte_groups, index + 1)) {
                    sum += number.parse::<u32>().unwrap();
                }
                number = String::new();
                counted = false;
            }
        }
    }
    sum
}

pub fn part1_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Must provide the path of the file to open");
    }

    let path = &args[2];
    let result: u32 = get_line_groups(get_lines(path), '.')
        .map(count_for_line_group)
        .sum();

    println!("{result}");
}

#[cfg(test)]
mod test {
    use crate::part1::count_for_line_group;

    #[test]
    fn eval_group_counted_before() {
        let group = [
            ".....".to_string(),
            "#123.".to_string(),
            ".....".to_string()
        ];
        assert_eq!(123, count_for_line_group(group));
    }

    #[test]
    fn eval_group_number_counted_above_before() {
        let group = [
            "#....".to_string(),
            ".123.".to_string(),
            ".....".to_string()
        ];
        assert_eq!(123, count_for_line_group(group));
    }

    #[test]
    fn eval_group_number_counted_below_mid() {
        let group = [
            ".....".to_string(),
            ".123.".to_string(),
            "..#..".to_string()
        ];
        assert_eq!(123, count_for_line_group(group));
    }
    
    #[test]
    fn eval_group_number_above_end() {
        let group = [
            "....#".to_string(),
            ".123.".to_string(),
            ".....".to_string()
        ];
        assert_eq!(123, count_for_line_group(group));
    }

    #[test]
    fn eval_group_number_below_end() {
        let group = [
            ".....".to_string(),
            ".123.".to_string(),
            "....#".to_string()
        ];
        assert_eq!(123, count_for_line_group(group));
    }

    #[test]
    fn eval_group_two_numbers() {
        let group = [
            ".....".to_string(),
            "12.3.".to_string(),
            "..#..".to_string()
        ];
        assert_eq!(15, count_for_line_group(group));
    }

    #[test]
    fn eval_group_missed_number() {
        let group = [
            ".....".to_string(),
            "12.3#".to_string(),
            ".....".to_string()
        ];
        assert_eq!(3, count_for_line_group(group));
    }
}