use crate::{get_lines::get_lines, map::{Direction, parse_map, Navigable}, lcm::lcm};

fn done(at: &Vec<String>) -> bool {
    at.iter().all(|a| a.ends_with("Z"))
}

fn debug_check(at: &Vec<String>, n: u32) {
    let which_at: Vec<_> = at.iter().map(|a| a.ends_with("Z")).collect();
    if which_at.iter().any(|b| *b) {
        print!("{n}: ");
        for w in which_at {
            let c = if w { 'O' } else { '.' };
            print!("{c}");
        }
        println!("");
    }
}

pub fn part2_main(args: &Vec<&String>) {
    if args.len() < 1 {
        panic!("Must provide the path of the file to open");
    }
    let path = args[0];
    let mut lines = get_lines(path);
    let directions = Direction::from_str(lines.next().unwrap().as_str());
    _ = lines.next();
    let map = parse_map(&mut lines);

    let mut result = 0u64;
    let mut index: usize = 0;
    let mut at: Vec<_> = map.keys().filter(|key| key.ends_with("A")).map(|key| key.to_owned()).collect();
    let mut dones: Vec<Vec<u64>> = at.iter().map(|_| Vec::new()).collect();
    while !done(&at) {
        let mut finished = false;
        for (idx, a) in at.iter().enumerate() {
            if a.ends_with("Z") {
                dones[idx].push(result as u64);
            }
            if dones.iter().all(|d| d.len() >= 2 && d.iter().all(|x| (x % (directions.len() as u64) == 0) && (x % d[0] == 0))) {
                result = dones.iter().map(|d| d.first().unwrap())
                    .fold(1u64, |acc, x| lcm(acc, *x));
                finished = true;
            }
        }
        if finished { 
            break;
        }
        let direction = directions[index];
        index = (index + 1) % directions.len();
        result += 1;
        for a in at.iter_mut() {
            *a = map.go(a, direction);
        }
    }

    println!("{result}");
}