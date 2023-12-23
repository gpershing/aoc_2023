use crate::{get_lines::get_lines, modules::ModuleNet, lcm::lcm};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut mods = ModuleNet::from_lines(get_lines(path));
    for i in 1..=50000 {
        mods.pulse_broadcast_print(i);
    }

    println!("{}", lcm(lcm(3907, 3911), lcm(3929, 4057)));
}
/*
jd: 3907, 7814, 11721, 15628 => 3907n
fv: 3911, 7822, 11733, 15644 => 3911n
lm: 3929, 7858, 11787, 15716 => 3929n
vm: 4057, 8114, 12171, 16228 => 4057n


*/