use crate::{get_lines::get_lines, race::Race};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let lines: Vec<_> = get_lines(path).take(2).collect();
    let races = Race::get_races(&lines[0], &lines[1]);
    let result: u64 = races.iter().map(Race::evaluate).product();

    println!("{result}");
}