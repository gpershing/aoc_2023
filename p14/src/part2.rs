use crate::{get_lines::get_lines, grid::Grid};

fn find_cycle(history: &Vec<(u64, usize)>, lookback: usize) -> Option<usize> {
    let lookback = lookback.min(history.len() - 1);
    let last = history.last().unwrap();
    for diff in 1..=lookback {
        let index = history.len() - diff - 1;
        if &history[index] == last {
            return Some(diff);
        }
    }
    None
}

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut grid = Grid::from_lines(&mut get_lines(path));
    let mut history = Vec::new();
    history.push((0, 0));
    let mut n = 0;
    let target = 1_000_000_000;
    let result = loop {
        n += 1;
        let next_grid = grid.slide_and_rotate().slide_and_rotate().slide_and_rotate().slide_and_rotate();
        history.push((next_grid.hash_u64(), next_grid.weight()));
        match find_cycle(&history, (n + 10) / 10) {
            Some(diff) => {
                let target_mod = target % diff;
                let m = history.len() / diff;
                let mut x = m * diff + target_mod;
                if x >= history.len() { x -= diff; }
                break history[x].1
            },
            None => ()
        }
        grid = next_grid;
    };

    println!("{result}");
}