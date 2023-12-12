use crate::get_lines::get_lines;

fn evaluate_line(line: String) -> u32 {
    todo!()
}

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let result: u32 = get_lines(path)
        .map(evaluate_line)
        .sum();

    println!("{result}");
}