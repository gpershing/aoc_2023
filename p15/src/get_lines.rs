use std::{io::{self, BufRead}, fs::File};

pub fn get_lines(path: &String) -> impl Iterator<Item=String> {
    let file = File::open(path).unwrap_or_else(|_| panic!("Could not open the given file"));
    io::BufReader::new(file).lines()
        .map(|line| line.unwrap())
}