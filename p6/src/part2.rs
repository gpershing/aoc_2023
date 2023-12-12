use crate::{get_lines::get_lines, race::Race};

fn remove_whitespace(s: &str) -> String {
    s.split_ascii_whitespace()
        .map(|s| s.to_string())
        .reduce(|acc, e| acc + &e)
        .unwrap()
}

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let lines: Vec<_> = get_lines(path).take(2)
        .map(|s| remove_whitespace(&s))
        .collect();
    let races = Race::get_races(&lines[0], &lines[1]);
    let result: u64 = races.iter().map(Race::evaluate).product();

    println!("{result}");
}