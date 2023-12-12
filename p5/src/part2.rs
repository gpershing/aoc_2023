use crate::{get_lines::get_lines, seeds::{get_seed_ranges, SeedRange}, map::Map};

fn evaluate(lines: &mut impl Iterator<Item = String>, seeds: Vec<SeedRange>) -> u64 {
    let mut current_values = seeds;
    while let Some(map) = Map::from_stream(lines) {
        current_values = current_values.iter().map(|input| map.map_range(input))
            .flatten().collect();
    }
    current_values.iter().min_by(|a, b| a.start.cmp(&b.start))
        .unwrap().start
}

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut lines = get_lines(path);
    let seeds = get_seed_ranges(lines.next().unwrap());
    let result = evaluate(&mut lines, seeds);

    println!("{result}");
}