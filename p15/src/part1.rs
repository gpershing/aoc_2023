use std::{fs::File, io::Read};

use crate::hash_f::hash_f;

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut buf = String::new();
    File::open(path).unwrap_or_else(|_| panic!("Could not open the given file")).read_to_string(&mut buf).unwrap();
    let result: u32 = buf.split(",").map(hash_f).map(|x| x as u32).sum();

    println!("{result}");
}