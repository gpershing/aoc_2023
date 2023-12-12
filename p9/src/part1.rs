use crate::{get_lines::get_lines, row::{Tableau, Row}};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let result: i32 = get_lines(path)
        .map(|line| Tableau::new(Row::from_line(line)))
        .map(|mut t| t.extend())
        .sum();

    println!("{result}");
}