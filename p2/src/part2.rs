use std::{env, collections::HashMap};

use crate::{game::{Game, Draw}, get_lines::get_lines};

fn max_with_bag(draw: &Draw, bag: &mut HashMap<String, u32>) {
    draw.color_draws().for_each(|color_draw| {
        let entry = bag.get_mut(color_draw.color).unwrap();
        *entry = (*entry).max(color_draw.count);
    });
}

fn power(line: String) -> u32 {
    let game = Game::new(line.as_str());
    let mut bag = HashMap::new();
    bag.insert("blue".to_string(), 0u32);
    bag.insert("red".into(), 0);
    bag.insert("green".into(), 0);

    game.draws().for_each(|draw| max_with_bag(&draw, &mut bag));

    bag.values().product()
}

pub fn part2_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Must provide the path of the file to open");
    }

    let path = &args[2];
    let result: u32 = get_lines(path)
        .map(|line| power(line))
        .sum();
    
    println!("{result}");
}

#[cfg(test)]
mod test {
    use crate::part2::power;

    #[test]
    fn get_power() {
        assert_eq!(6, power("Game 0: 1 red; 2 red, 1 blue; 3 blue, 1 green".to_string()));
    }

    #[test]
    fn get_power_zero() {
        assert_eq!(0, power("Game 0: 1 red, 2 blue; 5 blue, 14 red".to_string()));
    }
}