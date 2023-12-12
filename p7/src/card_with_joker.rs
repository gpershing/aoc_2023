#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JCard {
    J,
    N(u8),
    T,
    Q,
    K,
    A
}

impl JCard {
    pub fn from_char(c: char) -> Option<JCard> {
        match c {
            'A' => Some(JCard::A),
            'K' => Some(JCard::K),
            'Q' => Some(JCard::Q),
            'J' => Some(JCard::J),
            'T' => Some(JCard::T),
            _ => {
                if let Some(digit) = c.to_digit(10) {
                    if digit >= 2 {
                        Some(JCard::N(digit as u8))
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

    use super::JCard;

    #[test_case(JCard::J, JCard::N(2), Ordering::Less)]
    #[test_case(JCard::N(2), JCard::N(3), Ordering::Less)]
    #[test_case(JCard::N(3), JCard::N(2), Ordering::Greater)]
    #[test_case(JCard::N(2), JCard::N(2), Ordering::Equal)]
    #[test_case(JCard::N(9), JCard::T, Ordering::Less)]
    #[test_case(JCard::T, JCard::Q, Ordering::Less)]
    #[test_case(JCard::Q, JCard::K, Ordering::Less)]
    #[test_case(JCard::K, JCard::A, Ordering::Less)]
    fn test_order(a: JCard, b: JCard, ordering: Ordering) {
        assert_eq!(ordering, a.cmp(&b));
    }
}