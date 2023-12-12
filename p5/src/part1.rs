use crate::{get_lines::get_lines, seeds::get_seeds, map::Map};

fn evaluate(lines: &mut impl Iterator<Item = String>, seeds: &Vec<u64>) -> u64 {
    let mut current_values = seeds.clone();
    while let Some(map) = Map::from_stream(lines) {
        current_values.iter_mut().for_each(|entry| *entry = map.map(*entry));
    }
    *current_values.iter().min().unwrap()
}

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut lines = get_lines(path);
    let seeds = get_seeds(lines.next().unwrap());
    let result: u64 = evaluate(&mut lines, &seeds);

    println!("{result}");
}