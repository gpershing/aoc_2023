use crate::{get_lines::get_lines, map::{Direction, parse_map, Navigable}};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut lines = get_lines(path);
    let directions = Direction::from_str(lines.next().unwrap().as_str());
    _ = lines.next();
    let map = parse_map(&mut lines);

    let mut result = 0u32;
    let mut index: usize = 0;
    let mut at = "AAA".to_string();
    while at != "ZZZ" {
        let direction = directions[index];
        index = (index + 1) % directions.len();
        result += 1;
        at = map.go(&at, direction)
    }

    println!("{result}");
}