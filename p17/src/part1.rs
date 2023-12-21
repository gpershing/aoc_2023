use crate::{get_lines::get_lines, city_map::CityMap};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let result: u32 = CityMap::from_lines(&mut get_lines(path)).navigate_p1();

    println!("{result}");
}