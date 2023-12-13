use crate::{get_lines::get_lines, get_line_groups::get_line_groups, grid::Grid};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let result: usize = get_line_groups(&mut get_lines(path))
        .map(|lines| Grid::from_lines(&mut lines.into_iter()).mirror_eval_with_transpose())
        .sum();

    println!("{result}");
}