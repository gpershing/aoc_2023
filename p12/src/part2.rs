use crate::{get_lines::get_lines, row::Row, cache::new_cache};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut cache = new_cache();
    let result: usize = get_lines(path)
        .enumerate()
        .map(|(index, line)| { println!("{index}"); Row::from_line_expanded(line, 5).possibilities(&mut cache) })
        .sum();

    println!("{result}");
}