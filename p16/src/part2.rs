use crate::{get_lines::get_lines, puzzle::Puzzle};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let result = Puzzle::from_lines(&mut get_lines(path)).eval_max();

    println!("{result}");
}