use crate::{get_lines::get_lines, dig_instruction::DigInstruction, dig::dig_count};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let result = dig_count(get_lines(path).map(|line| DigInstruction::from_hex(&line)));

    println!("{result}");
}