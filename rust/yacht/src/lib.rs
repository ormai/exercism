use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let freqs = dice.iter().fold(HashMap::new(), |mut map, dice| {
        *map.entry(dice).or_insert(0u8) += 1;
        map
    });

    match category {
        Category::Ones => *freqs.get(&1).unwrap_or(&0),
        Category::Twos => *freqs.get(&2).unwrap_or(&0) * 2,
        Category::Threes => *freqs.get(&3).unwrap_or(&0) * 3,
        Category::Fours => *freqs.get(&4).unwrap_or(&0) * 4,
        Category::Fives => *freqs.get(&5).unwrap_or(&0) * 5,
        Category::Sixes => *freqs.get(&6).unwrap_or(&0) * 6,
        Category::FullHouse => {
            if freqs.len() == 2 && (2..=3).contains(freqs.values().next().unwrap()) {
                freqs.iter().map(|(&k, v)| k * v).sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => freqs
            .iter()
            .find(|(_, v)| **v >= 4)
            .map_or(0, |(&k, _)| k * 4),
        Category::LittleStraight => {
            if freqs.len() == 5 && !freqs.contains_key(&6) {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if freqs.len() == 5 && !freqs.contains_key(&1) {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if freqs.len() == 1 {
                50
            } else {
                0
            }
        }
    }
}
