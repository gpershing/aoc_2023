use crate::{get_lines::get_lines, map::Map};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let result = Map::from_lines(get_lines(path)).to_loop_map().count_inside();

    println!("{result}");
}