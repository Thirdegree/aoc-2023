#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

#[allow(dead_code)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl TryFrom<&str> for Card {
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (id_str, numbers) = value.split_once(": ").unwrap();
        let id = id_str.strip_prefix("Card ").unwrap();
        let (winning_numbers, card_numbers) = numbers.split_once('|').unwrap();
        Ok(Self {
            id: id.trim().parse()?,
            winning_numbers: winning_numbers
                .split_whitespace()
                .map(|n| n.trim().parse())
                .collect::<anyhow::Result<_, Self::Error>>()?,
            card_numbers: card_numbers
                .split_whitespace()
                .map(|n| n.trim().parse())
                .collect::<Result<_, Self::Error>>()?,
        })
    }
}

impl Card {
    fn winning_numbers(&self) -> HashSet<&u32> {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .collect()
    }
}

fn main() -> anyhow::Result<()> {
    let cards: Vec<Card> = aoc_2023::include_data!(day4)
        .lines()
        .map(TryInto::try_into)
        .collect::<Result<_, std::num::ParseIntError>>()?;
    let winning_number_counts: Vec<_> = cards.iter().map(|c| c.winning_numbers().len()).collect();
    let mut count_cards_remain = vec![1; cards.len()];
    for (idx, count) in winning_number_counts.iter().enumerate() {
        for n in idx + 1..=idx + count {
            count_cards_remain[n] += count_cards_remain[idx];
        }
    }
    println!("Day 4 result: {}", count_cards_remain.iter().sum::<u32>());
    Ok(())
}
