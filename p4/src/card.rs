pub struct Card {
    pub index: u32,
    pub winning_numbers: Vec<u32>,
    pub my_numbers: Vec<u32>
}

impl Card {
    fn get_numbers(s: &str) -> Vec<u32> {
        s.trim().split_ascii_whitespace()
            .map(|number_str| number_str.parse().unwrap())
            .collect()
    }

    pub fn new(game: &str) -> Self {
        let parts: Vec<&str> = game.split(":").collect();
        if parts.len() != 2 {
            panic!("Unexpected card format")
        }
    
        let index_part = &parts[0];
        if !index_part.starts_with("Card ") {
            panic!("Missing Card specifier");
        }
        let index_number_part = &index_part["Game ".len()..index_part.len()];
        let index = index_number_part.trim().parse::<u32>().unwrap();

        let number_parts: Vec<&str> = parts[1].split('|').collect();
        if number_parts.len() != 2 {
            panic!("Unexpected numbers format")
        }

        let winning_numbers = Card::get_numbers(number_parts[0]);
        let my_numbers = Card::get_numbers(number_parts[1]);
    
        Card { index, winning_numbers, my_numbers }
    }

    pub fn match_count(&self) -> u32 {
        self.my_numbers.iter()
            .filter(|my_number| self.winning_numbers.contains(my_number))
            .count() as u32
    }

    pub fn score(&self) -> u32 {
        let count = self.match_count();
        if count == 0 {
            0
        }
        else {
            2u32.pow(count - 1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Card;
    use test_case::test_case;

    #[test]
    fn parse_empty_card() {
        let card = Card::new("Card 1: |");
        assert_eq!(1, card.index);
        assert_eq!(0, card.winning_numbers.len());
        assert_eq!(0, card.my_numbers.len());
    }

    #[test]
    fn parse_single_number_card() {
        let card = Card::new("Card 1: 1 | 2");
        assert_eq!(1, card.index);
        assert_eq!(1, card.winning_numbers.len());
        assert_eq!(1, card.winning_numbers[0]);
        assert_eq!(1, card.my_numbers.len());
        assert_eq!(2, card.my_numbers[0]);
    }


    #[test]
    fn parse_multiple_number_card() {
        let card = Card::new("Card 1: 10 20 | 10 20 30");
        assert_eq!(1, card.index);
        assert_eq!(2, card.winning_numbers.len());
        assert_eq!(10, card.winning_numbers[0]);
        assert_eq!(20, card.winning_numbers[1]);
        assert_eq!(3, card.my_numbers.len());
        assert_eq!(10, card.my_numbers[0]);
        assert_eq!(20, card.my_numbers[1]);
        assert_eq!(30, card.my_numbers[2]);
    }

    #[test]
    fn parse_extra_whitespace_card() {
        let card = Card::new("Card 1:  1  2   |  3      4 ");
        assert_eq!(1, card.index);
        assert_eq!(2, card.winning_numbers.len());
        assert_eq!(1, card.winning_numbers[0]);
        assert_eq!(2, card.winning_numbers[1]);
        assert_eq!(2, card.my_numbers.len());
        assert_eq!(3, card.my_numbers[0]);
        assert_eq!(4, card.my_numbers[1]);
    }

    #[test_case("Card 1: | ", 0 ; "empty card")]
    #[test_case("Card 1: 1 | ", 0 ; "empty my numbers")]
    #[test_case("Card 1: | 1 ", 0 ; "empty winning numbers")]
    #[test_case("Card 1: 1 | 2", 0 ; "one miss")]
    #[test_case("Card 1: 1 2 | 3 4", 0 ; "multiple misses")]
    #[test_case("Card 1: 1 | 1 ", 1 ; "one hit")]
    #[test_case("Card 1: 1 2 | 1 2 ", 2 ; "two hits")]
    #[test_case("Card 1: 1 2 3 | 2 5 4 3 1 ", 4 ; "multiple hits")]
    #[test_case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8 ; "real example 1")]
    #[test_case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2 ; "real example 2")]
    #[test_case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0 ; "real example 3")]
    fn test_card_score(card: &str, score: u32) {
        let card = Card::new(card);
        assert_eq!(score, card.score());
    }
}