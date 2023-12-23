use std::env;

use part1::part1_main;
use part2::part2_main;

mod get_lines;
mod part1;
mod part2;
mod modules;
mod lcm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must provide which part to use");
    }
    let part: u32 = args[1].parse().unwrap();
    match part {
        1 => part1_main(&args[2..].into_iter().collect()),
        2 => part2_main(&args[2..].into_iter().collect()),
        _ => panic!("Please select part 1 or 2")
    };
}
