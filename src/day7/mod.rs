use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_while_m_n},
    character::{
        complete::{alphanumeric1, digit1, line_ending, one_of, space1},
        is_digit,
    },
    combinator::{map, map_res, value},
    complete::take,
    error::Error,
    multi::{many1, separated_list1},
    number,
    sequence::{self, separated_pair},
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Debug, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N(u8),
}

impl Card {
    pub fn value(&self) -> u8 {
        match &self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::N(n) => *n,
        }
    }

    pub fn parse(input: char) -> Option<Card> {
        match input {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            c if c.is_digit(10) => Some(Card::N(c.to_digit(10).unwrap() as u8)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Kind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl Kind {
    pub fn rank(&self) -> usize {
        match self {
            Kind::FiveOfAKind => 7,
            Kind::FourOfAKind => 6,
            Kind::FullHouse => 5,
            Kind::ThreeOfAKind => 4,
            Kind::TwoPairs => 3,
            Kind::OnePair => 2,
            Kind::HighCard => 1,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        let mut cards = cards;
        cards.sort_by_key(|c| 20 - c.value());

        Self { cards }
    }

    pub fn parse(input: &str) -> Self {
        let cards: Vec<Card> = input.chars().map(Card::parse).flatten().collect();
        Hand::new(cards)
    }

    pub fn kind(&self) -> Kind {
        let matching_pairs: u8 = self
            .cards
            .windows(2)
            .map(|w| if w[0].value() == w[1].value() { 1 } else { 0 })
            .sum();

        // println!("Comparing {:?} with pairs: {}", self.cards, matching_pairs);

        match matching_pairs {
            4 => Kind::FiveOfAKind,
            3 if self.cards.get(0).unwrap().value() == self.cards.get(3).unwrap().value() => {
                Kind::FourOfAKind
            }
            3 => Kind::FullHouse,
            2 if self.cards.get(0).unwrap().value() == self.cards.get(2).unwrap().value() => {
                Kind::ThreeOfAKind
            }
            2 => Kind::TwoPairs,
            1 => Kind::OnePair,
            _ => Kind::HighCard,
        }
    }

    pub fn card_value(&self) -> usize {
        self.cards.iter().map(|c| c.value() as usize).sum()
    }

    pub fn value(&self) -> usize {
        &self.kind().rank() * 1000 + &self.card_value()
    }
}

fn parse(input: &str) -> Vec<(Hand, usize)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();

            let hand = Hand::parse(parts[0]);

            let value: usize = parts[1].parse().unwrap();

            return (hand, value);
        })
        .collect()
}

pub fn process(input: String) {
    let mut table = parse(&input);
    table.sort_by_cached_key(|(h, bet)| h.value());

    let result: usize = table
        .iter()
        .enumerate()
        .map(|(i, (h, b))| {
            println!("Hand {i} bet {b}");
            return b * (i + 1);
        })
        .sum();

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_kind() {
        assert_eq!(Hand::parse("AAAAA").kind(), Kind::FiveOfAKind);
        assert_eq!(Hand::parse("AATAA").kind(), Kind::FourOfAKind);
        assert_eq!(Hand::parse("33322").kind(), Kind::FullHouse);
        assert_eq!(Hand::parse("33321").kind(), Kind::ThreeOfAKind);
        assert_eq!(Hand::parse("6565Q").kind(), Kind::TwoPairs);
    }
}
