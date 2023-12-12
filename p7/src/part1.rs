use crate::{get_lines::get_lines, hand::HandWithBid};

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut hands: Vec<_> = get_lines(path)
        .map(|line| HandWithBid::from_str(line.as_str()))
        .collect();
    hands.sort_by(|a, b| a.hand.cmp(&b.hand));
    let result: u64 = hands.iter().enumerate()
        .map(|(index, hand)| (index as u64 + 1) * hand.bid)
        .sum();

    println!("{result}");
}