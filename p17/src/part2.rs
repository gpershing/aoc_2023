use crate::{get_lines::get_lines, ultra::UltraCityMap};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let result: u32 = UltraCityMap::from_lines(&mut get_lines(path)).navigate_p();

    println!("{result}");
}