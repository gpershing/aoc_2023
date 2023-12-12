use std::{env, collections::VecDeque};

use crate::{get_lines::get_lines, card::Card};

fn score_line(line: String, copies_queue: &mut VecDeque<u32>) -> u32 {
    let copies = copies_queue.pop_front().unwrap_or(0) + 1;

    let matches = Card::new(line.as_str()).match_count() as usize;
    while copies_queue.len() < matches {
        copies_queue.push_back(0);
    }
    (0..matches).for_each(|index| copies_queue[index] += copies);

    copies
}

pub fn part2_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Must provide the path of the file to open");
    }

    let path = &args[2];
    let mut copies = VecDeque::new();
    let result: u32 = get_lines(path)
        .map(|line| score_line(line, &mut copies))
        .sum();

    println!("{result}");
}