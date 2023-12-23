use crate::{get_lines::get_lines, plot::Plot};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 2 {
        panic!("Must provide the path of the file to open and the desired step count");
    }
    let path = args[0];
    let steps: u32 = args[1].parse().unwrap();
    let plot = Plot::from_lines(get_lines(path));
    let dists = plot.get_distances();
    let result = dists.values().filter(|v| *v % 2 == steps % 2 && **v <= steps).count();

    println!("{result}");
}