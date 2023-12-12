use std::env;

use crate::{get_lines::get_lines, card::Card};

pub fn part1_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Must provide the path of the file to open");
    }

    let path = &args[2];
    let result: u32 = get_lines(path)
        .map(|s| Card::new(s.as_str()).score())
        .sum();

    println!("{result}");
}