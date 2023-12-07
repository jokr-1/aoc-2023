use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(s: char) -> Self {
        match s {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unknown card: {s}"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let set = HashSet::from(self.cards);
        let mut counts = set
            .iter()
            .map(|v| self.cards.iter().filter(|&a| a == v).count())
            .collect::<Vec<usize>>();

        counts.sort();

        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            x => panic!("Unexpected Hand: {x:?}"),
        }
    }
}

pub fn solve(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            Hand {
                cards: cards
                    .chars()
                    .map(Card::from_char)
                    .collect::<Vec<Card>>()
                    .try_into()
                    .expect("Can't collect to array"),
                bid: bid.parse().unwrap(),
            }
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}
