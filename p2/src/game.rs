pub struct Game<'a> {
    pub index: u32,
    pub draws_str: &'a str
}

impl<'a> Game<'a> {
    pub fn new(game: &'a str) -> Self {
        let parts: Vec<&str> = game.split(":").collect();
        if parts.len() != 2 {
            panic!("Unexpected game format")
        }
    
        let game_part = &parts[0];
        if !game_part.starts_with("Game ") {
            panic!("Missing Game specifier");
        }
        let game_number_part = &game_part["Game ".len()..game_part.len()];
        let game_number = game_number_part.parse::<u32>().unwrap();

        Game { index: game_number, draws_str: parts[1] }
    }

    pub fn draws(&self) -> impl Iterator<Item=Draw<'a>> {
        self.draws_str.split(";")
            .map(|draw| Draw { draw })
    }
}

pub struct Draw<'a> {
    pub draw: &'a str
}

impl<'a> Draw<'a> {
    pub fn color_draws(&self) -> impl Iterator<Item=ColorDraw<'a>> {
        self.draw.split(",").map(ColorDraw::new)
    }
}

pub struct ColorDraw<'a> {
    pub color: &'a str,
    pub count: u32
}

impl<'a> ColorDraw<'a> {
    pub fn new(draw_str: &'a str) -> Self {
        let parts: Vec<&str> = draw_str.trim().split(" ").collect();
        if parts.len() != 2 {
            panic!("Unexpected color draw format");
        }

        let count = parts[0].parse().unwrap();
        
        ColorDraw { color: parts[1], count }
    }
}