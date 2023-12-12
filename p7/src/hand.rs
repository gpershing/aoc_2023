use crate::card::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    hand: [Card; 5]
}

impl Hand {
    pub fn from_str(s: &str) -> Option<Hand> {
        let v: Vec<_> = s.chars().filter_map(Card::from_char).take(5).collect();
        if v.len() >= 5 {
            Some(Hand { hand: [v[0], v[1], v[2], v[3], v[4]] })
        }
        else {
            None
        }
    }

    pub fn counts(&self) -> Vec<(Card, u8)> {
        let mut counts: Vec<(Card, u8)> = Vec::new();
        for card in self.hand {
            if let Some(entry) = counts.iter_mut().find(|entry| entry.0 == card) {
                entry.1 += 1;
            }
            else {
                counts.push((card, 1u8));
            }
        }
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        counts
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let counts_a: Vec<_> = self.counts().into_iter().map(|entry| entry.1).collect();
        let counts_b: Vec<_> = other.counts().into_iter().map(|entry| entry.1).collect();
        match counts_a.cmp(&counts_b) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => self.hand.cmp(&other.hand),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct HandWithBid {
    pub hand: Hand,
    pub bid: u64
}

impl HandWithBid {
    pub fn from_str(s: &str) -> HandWithBid {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();
        if parts.len() < 2 {
            panic!("Unexpected format");
        }
        let hand = Hand::from_str(parts[0]).unwrap();
        let bid = parts[1].parse().unwrap();
        HandWithBid { hand, bid }
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use test_case::test_case;

    use super::Hand;

    #[test_case(Hand::from_str("22222").unwrap(), Hand::from_str("AAAAK").unwrap(), Ordering::Greater)]
    #[test_case(Hand::from_str("22223").unwrap(), Hand::from_str("AAAKK").unwrap(), Ordering::Greater)]
    #[test_case(Hand::from_str("22233").unwrap(), Hand::from_str("AAAKQ").unwrap(), Ordering::Greater)]
    #[test_case(Hand::from_str("22234").unwrap(), Hand::from_str("AAKKQ").unwrap(), Ordering::Greater)]
    #[test_case(Hand::from_str("22334").unwrap(), Hand::from_str("AAKQJ").unwrap(), Ordering::Greater)]
    #[test_case(Hand::from_str("22345").unwrap(), Hand::from_str("AKQJT").unwrap(), Ordering::Greater)]
    #[test_case(Hand::from_str("AAAAA").unwrap(), Hand::from_str("KKKKK").unwrap(), Ordering::Greater)]
    #[test_case(Hand::from_str("AKKKK").unwrap(), Hand::from_str("KAAAA").unwrap(), Ordering::Greater)]
    #[test_case(Hand::from_str("KAQJK").unwrap(), Hand::from_str("KAQTA").unwrap(), Ordering::Greater)]
    fn test_cmp(a: Hand, b: Hand, ordering: Ordering) {
        assert_eq!(ordering, a.cmp(&b));
    }
}