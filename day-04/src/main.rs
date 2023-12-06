use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

struct Card {
    winning: HashSet<u8>,
    hand: HashSet<u8>,
}

impl Card {
    fn points(&self) -> u32 {
        match self.winner_count() {
            0 => 0,
            n => u32::pow(2, n as u32 - 1),
        }
    }

    fn winner_count(&self) -> u8 {
        self.winning.intersection(&self.hand).count() as u8
    }
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('|').unwrap();
        let (_, left) = left.split_once(':').unwrap();

        let parse_number = |s| match s {
            " " | "|" => None,
            _ => Some(s.parse::<u8>().unwrap()),
        };

        let winning = left.split_whitespace().filter_map(parse_number).collect();
        let hand = right.split_whitespace().filter_map(parse_number).collect();

        Ok(Card { winning, hand })
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<Card>().unwrap().points())
        .sum()
}

fn part_two(input: &str) -> u32 {
    let cards: Vec<Card> = input.lines().map(|l| l.parse::<Card>().unwrap()).collect();
    let mut card_counts = vec![1u32; cards.len()];
    let n_cards = cards.len();
    for (i, card) in cards.iter().enumerate() {
        let p = card.winner_count();
        let n = card_counts[i];
        card_counts[(i + 1).min(n_cards)..(i + p as usize + 1).min(n_cards)]
            .iter_mut()
            .for_each(|c| *c += n);
    }
    card_counts.iter().sum()
}
