use crate::{get_lines::get_lines, workflow::WorkflowNet};

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut lines = get_lines(path);
    let workflow = WorkflowNet::from_lines(&mut lines);
    let result = workflow.sum_all();

    println!("{result}");
}