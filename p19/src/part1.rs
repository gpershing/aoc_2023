use crate::{get_lines::get_lines, workflow::WorkflowNet, part::Part};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut lines = get_lines(path);
    let workflow = WorkflowNet::from_lines(&mut lines);
    let result: u32 = lines.map(|line| workflow.sum_if_accepted(&Part::from_str(&line))).sum();

    println!("{result}");
}