use std::{fs::File, io::Read};

use crate::hash_f_map::HashFMap;

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut buf = String::new();
    File::open(path).unwrap_or_else(|_| panic!("Could not open the given file")).read_to_string(&mut buf).unwrap();
    let mut map = HashFMap::new();
    buf.split(",").map(|s| s.replace('\n', "")).for_each(|s| {
        if s.contains("=") {
            let parts: Vec<_> = s.split("=").collect();
            map.eq(parts[0], parts[1].parse().unwrap());
        }
        else {
            map.sub(&s[0..s.len() - 1])
        }
    });
    let result = map.total_power();

    println!("{result}");
}