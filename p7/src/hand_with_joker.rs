use crate::card_with_joker::JCard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JHand {
    hand: [JCard; 5]
}

impl JHand {
    pub fn from_str(s: &str) -> Option<JHand> {
        let v: Vec<_> = s.chars().filter_map(JCard::from_char).take(5).collect();
        if v.len() >= 5 {
            Some(JHand { hand: [v[0], v[1], v[2], v[3], v[4]] })
        }
        else {
            None
        }
    }

    pub fn counts(&self) -> Vec<(JCard, u8)> {
        let mut counts: Vec<(JCard, u8)> = Vec::new();
        let mut jcount = 0u8;
        for card in self.hand {
            if card == JCard::J {
                jcount += 1;
            }
            else {
                if let Some(entry) = counts.iter_mut().find(|entry| entry.0 == card) {
                    entry.1 += 1;
                }
                else {
                    counts.push((card, 1u8));
                }
            }
        }
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        if counts.len() == 0 {
            vec![(JCard::J, 5)]
        }
        else {
            counts[0].1 += jcount;
            counts
        }
    }
}

impl Ord for JHand {
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

impl PartialOrd for JHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct JHandWithBid {
    pub hand: JHand,
    pub bid: u64
}

impl JHandWithBid {
    pub fn from_str(s: &str) -> JHandWithBid {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();
        if parts.len() < 2 {
            panic!("Unexpected format");
        }
        let hand = JHand::from_str(parts[0]).unwrap();
        let bid = parts[1].parse().unwrap();
        JHandWithBid { hand, bid }
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use test_case::test_case;

    use super::JHand;

    #[test_case(JHand::from_str("22222").unwrap(), JHand::from_str("AAAAK").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("22223").unwrap(), JHand::from_str("AAAKK").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("22233").unwrap(), JHand::from_str("AAAKQ").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("22234").unwrap(), JHand::from_str("AAKKQ").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("22334").unwrap(), JHand::from_str("AAKQT").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("22345").unwrap(), JHand::from_str("AKQT9").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("AAAAA").unwrap(), JHand::from_str("KKKKK").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("AKKKK").unwrap(), JHand::from_str("KAAAA").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("KAQTK").unwrap(), JHand::from_str("KAQ9A").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("22JJJ").unwrap(), JHand::from_str("AAAAK").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("JJJJJ").unwrap(), JHand::from_str("AAAAK").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("22222").unwrap(), JHand::from_str("JJJJJ").unwrap(), Ordering::Greater)]
    #[test_case(JHand::from_str("223JJ").unwrap(), JHand::from_str("AAAKK").unwrap(), Ordering::Greater)]
    fn test_order(a: JHand, b: JHand, ordering: Ordering) {
        assert_eq!(ordering, a.cmp(&b));
    }
}