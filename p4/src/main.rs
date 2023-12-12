use std::env;

use part1::part1_main;
use part2::part2_main;

mod part2;
mod part1;
mod get_lines;
mod card;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must provide a part number");
    }
    let part: u32 = args[1].parse().unwrap();
    match part {
        1 => part1_main(),
        2 => part2_main(),
        _ => panic!()
    };
}
