use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn parse(input: &str) -> Self {
        let cards: Vec<Card> = input.chars().map(Card::parse).flatten().collect();
        Hand::new(cards)
    }

    pub fn kind(&self) -> (Kind, u8) {
        let mut ordered_cards = self.cards;
        ordered_cards.sort_by_key(|c| c.value());

        let matching_tripples: Vec<_> = ordered_cards
            .windows(3)
            .filter(|w| w[0].value() == w[1].value() && w[1].value() == w[2].value())
            .collect();

        let tripple_value = matching_tripples.first().map(|w| w[0].value()).unwrap_or(0);
        let tripple_count = matching_tripples.len();

        let matching_pairs: Vec<_> = ordered_cards
            .windows(2)
            .filter(|w| w[0].value() == w[1].value())
            .collect();

        let pair_value = matching_pairs.first().map(|w| w[0].value()).unwrap_or(0);
        let pair_count = matching_pairs.len();

        match (tripple_count, pair_count) {
            (3, _) => (Kind::FiveOfAKind, tripple_value),
            (2, _) => (Kind::FourOfAKind, tripple_value),
            (1, 3) => (Kind::FullHouse, tripple_value),
            (1, _) => (Kind::ThreeOfAKind, tripple_value),
            (0, 2) => (Kind::TwoPairs, pair_value),
            (0, 1) => (Kind::OnePair, pair_value),
            _ => (Kind::HighCard, 0),
        }
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (kind, _) = self.kind();
        let (other_kind, _) = other.kind();

        if kind.rank() == other_kind.rank() {
            for (card, other_card) in self.cards.iter().zip(&other.cards) {
                if card.value() != other_card.value() {
                    return card.value().cmp(&other_card.value());
                }
            }
            return std::cmp::Ordering::Equal;
        } else {
            return kind.rank().cmp(&other_kind.rank());
        }
    }

    pub fn card_value(&self) -> usize {
        self.cards
            .iter()
            .enumerate()
            .map(|(i, c)| c.value() as usize * 10usize.pow(i as u32 + 1))
            .sum()
    }

    pub fn value(&self) -> usize {
        let (kind, dominant_card) = self.kind();
        let value = kind.rank() * 1_000_000_0000
            + dominant_card as usize * 10_000_0000
            + &self.card_value();
        return value;
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
    table.sort_by(|(a, _), (b, _)| a.cmp(b));

    let result: usize = table
        .iter()
        .enumerate()
        .map(|(i, (h, b))| {
            println!("{i}: Hand {h:?} bet {b}");
            return b * (i + 1);
        })
        .sum();

    // 249788985 is too high
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_kind() {
        assert_eq!(Hand::parse("AAAAA").kind(), (Kind::FiveOfAKind, 14));
        assert_eq!(Hand::parse("AATAA").kind(), (Kind::FourOfAKind, 14));
        assert_eq!(Hand::parse("33322").kind(), (Kind::FullHouse, 3));
        assert_eq!(Hand::parse("33321").kind(), (Kind::ThreeOfAKind, 3));
        assert_eq!(Hand::parse("6565Q").kind(), (Kind::TwoPairs, 6));
        assert_eq!(Hand::parse("333AQ").kind(), (Kind::ThreeOfAKind, 3));
        assert_eq!(Hand::parse("23456").kind(), (Kind::HighCard, 0))
    }

    #[test]
    fn hand_order() {
        assert_eq!(
            Hand::parse("22345").value() > Hand::parse("AQTJ9").value(),
            true
        );
    }
}
