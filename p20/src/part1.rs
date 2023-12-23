use crate::{get_lines::get_lines, modules::ModuleNet};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut mods = ModuleNet::from_lines(get_lines(path));
    let (lo, hi) = (0..1000).fold((0, 0), |(lo, hi), _| {
        let (next_lo, next_hi) = mods.pulse_broadcast();
        (lo + next_lo, hi + next_hi)
    });

    println!("{lo} {hi}");
    println!("{}", lo * hi);
}