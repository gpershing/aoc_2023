use std::{env, collections::HashMap};

use crate::{get_lines::get_lines, game::{Game, Draw, ColorDraw}};

fn is_possible_color_draw(color_draw: &ColorDraw, bag: &HashMap<String, u32>) -> bool {
    bag.get(color_draw.color).map(|max| color_draw.count <= *max).unwrap_or(false)
}

fn is_possible_draw(draw: &Draw, bag: &HashMap<String, u32>) -> bool {
    draw.color_draws().all(|color_draw| is_possible_color_draw(&color_draw, bag))
}

fn is_possible_game(game: &Game, bag: &HashMap<String, u32>) -> bool {
    game.draws().all(|draw| is_possible_draw(&draw, bag))
}

fn possible_game_number(line: String, bag: &HashMap<String, u32>) -> Option<u32> {
    let game = Game::new(line.as_str());

    if is_possible_game(&game, &bag) { Some(game.index) } else { None }
}

pub fn part1_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Must provide the path of the file to open");
    }

    let mut bag = HashMap::new();
    bag.insert("red".to_string(), 12u32);
    bag.insert("green".into(), 13);
    bag.insert("blue".into(), 14);

    let path = &args[2];
    let result: u32 = get_lines(path)
        .filter_map(|line| possible_game_number(line, &bag))
        .sum();

    println!("{result}");
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::part1::{possible_game_number};

    #[test]
    fn possible_game_number_ok() {
        let mut bag = HashMap::new();
        bag.insert("blue".to_string(), 4u32);
        bag.insert("red".to_string(), 5);
        assert_eq!(Some(1), possible_game_number("Game 1: 4 blue, 1 red; 1 blue, 5 red".to_string(), &bag));
    }
    
    #[test]
    fn possible_game_number_bad() {
        let mut bag = HashMap::new();
        bag.insert("blue".to_string(), 4u32);
        bag.insert("red".to_string(), 5);
        assert_eq!(None, possible_game_number("Game 1: 4 blue, 1 red; 1 blue, 6 red".to_string(), &bag));
    }
}