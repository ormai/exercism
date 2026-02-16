use std::collections::{BinaryHeap, HashMap};

/// Given a list of poker hands, return a list of those hands which win.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands = BinaryHeap::from_iter(hands.iter().map(|&s| (Hand::parse(s), s)));
    let (winning, _) = hands.peek().unwrap();
    Vec::from_iter(
        hands
            .iter()
            .take_while(|(value, _)| value == winning)
            .map(|&(_, s)| s),
    )
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    counts: Vec<usize>,
    values: Vec<u8>,
}

impl Hand {
    fn parse(card: &str) -> Self {
        let (ranks, suits): (Vec<u8>, Vec<u8>) = card
            .split_whitespace()
            .map(|card| {
                let (value, suit) = card.split_at(card.len() - 1);
                (
                    value
                        .parse()
                        .unwrap_or_else(|_| "JQKA".find(value).unwrap() as u8 + 11),
                    suit.as_bytes()[0],
                )
            })
            .unzip();
        let mut groups = HashMap::new();
        for v in ranks {
            *groups.entry(v).or_insert(0) += 1;
        }
        let mut groups = Vec::from_iter(groups.into_iter().map(|(v, c)| (c, v)));
        groups.sort_unstable_by(|a, b| b.cmp(a));
        let (mut counts, mut values): (Vec<_>, Vec<_>) = groups.iter().copied().unzip();
        if counts.len() == 5 {
            if values == [14, 5, 4, 3, 2] {
                values = vec![5, 4, 3, 2, 1];
            }
            let is_straight = values[0] - values[4] == 4;
            let is_flush = suits[1..].iter().all(|&x| x == suits[0]);
            match (is_straight, is_flush) {
                (true, true) => counts = vec![5],
                (true, false) => counts = vec![3, 1, 2],
                (false, true) => counts = vec![3, 1, 3],
                _ => {}
            }
        }
        Self { counts, values }
    }
}
