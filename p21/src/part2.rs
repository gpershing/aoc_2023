use crate::{get_lines::get_lines, plot::Plot};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 2 {
        panic!("Must provide the path of the file to open and the desired step count");
    }
    let path = args[0];
    let steps: u32 = args[1].parse().unwrap();
    let plot = Plot::from_lines(get_lines(path));
    let result = plot.get_steps_repeating(steps);

    println!("{result}");
}