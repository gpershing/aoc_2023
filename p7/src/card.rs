#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    N(u8),
    T,
    J,
    Q,
    K,
    A
}

impl Card {
    pub fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            _ => {
                if let Some(digit) = c.to_digit(10) {
                    if digit >= 2 {
                        Some(Card::N(digit as u8))
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use test_case::test_case;

    use super::Card;

    #[test_case(Card::N(2), Card::N(3), Ordering::Less)]
    #[test_case(Card::N(3), Card::N(2), Ordering::Greater)]
    #[test_case(Card::N(2), Card::N(2), Ordering::Equal)]
    #[test_case(Card::N(9), Card::T, Ordering::Less)]
    #[test_case(Card::T, Card::J, Ordering::Less)]
    #[test_case(Card::J, Card::Q, Ordering::Less)]
    #[test_case(Card::Q, Card::K, Ordering::Less)]
    #[test_case(Card::K, Card::A, Ordering::Less)]
    fn test_order(a: Card, b: Card, ordering: Ordering) {
        assert_eq!(ordering, a.cmp(&b));
    }
}