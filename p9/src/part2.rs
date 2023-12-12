use crate::{get_lines::get_lines, row::{Tableau, Row}};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let result: i32 = get_lines(path)
        .map(|line| Tableau::new(Row::from_line(line)))
        .map(|mut t: Tableau| t.prepend())
        .sum();

    println!("{result}");
}