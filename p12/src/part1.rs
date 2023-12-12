use crate::{get_lines::get_lines, row::Row, cache::new_cache};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut cache = new_cache();
    let result: usize = get_lines(path)
        .map(|line| Row::from_line(line).possibilities(&mut cache))
        .sum();

    println!("{result}");
}