use crate::{get_lines::get_lines, galaxy_map::get_galaxy_coords};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let galaxies = get_galaxy_coords(&mut get_lines(path), 1000000);
    let result: i64 = (0..galaxies.len() - 1).map(|first| {
        (first+1..galaxies.len()).map(|second| 
            galaxies[first].taxicab_dist(&galaxies[second])
        ).sum::<i64>()
    }).sum();

    println!("{result}");
}