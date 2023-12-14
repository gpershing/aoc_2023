use crate::{get_lines::get_lines, tile::Tile};

fn evaluate_line(line: String) -> u32 {
    todo!()
}

pub fn part1_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let (_, sum, count, row_count) = get_lines(path).enumerate().fold((None, 0, 0, 0), |(hit, mut sum, mut count, mut row_count), (row_index, line)| {
        let mut hit: Vec<u32> = hit.unwrap_or_else(|| (0..line.len()).map(|_| 0u32).collect());
        row_count += 1;
        line.char_indices().for_each(|(index, c)| {
            match Tile::from_char(c) {
                Some(Tile::Circle) => {
                    count += 1;
                    sum += hit[index];
                    hit[index] += 1;
                },
                Some(Tile::Square) => {
                    hit[index] = row_index as u32 + 1;
                },
                None => (),
            }
        });
        (Some(hit), sum, count, row_count)
    });
    let val = (count * row_count) - sum;

    println!("{val}");
}