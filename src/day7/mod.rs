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

    pub fn value2(&self) -> u8 {
        match &self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 1,
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

#[derive(Debug, PartialEq, PartialOrd)]
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

#[derive(Debug, PartialEq)]
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

    pub fn replace_joker(&self, card: Card) -> Hand {
        let cards = self
            .cards
            .iter()
            .map(|c| {
                if c.value2() == Card::J.value2() {
                    card.clone()
                } else {
                    c.clone()
                }
            })
            .collect();
        return Hand::new(cards);
    }

    pub fn kind(&self) -> Kind {
        let mut ordered_cards = self.cards.to_owned();
        ordered_cards.sort_by_key(|c| c.value());

        let matching_tripples = ordered_cards
            .windows(3)
            .filter(|w| w[0].value() == w[1].value() && w[1].value() == w[2].value())
            .count();

        let matching_pairs = ordered_cards
            .windows(2)
            .filter(|w| w[0].value() == w[1].value())
            .count();

        match (matching_tripples, matching_pairs) {
            (3, _) => Kind::FiveOfAKind,
            (2, _) => Kind::FourOfAKind,
            (1, 3) => Kind::FullHouse,
            (1, _) => Kind::ThreeOfAKind,
            (0, 2) => Kind::TwoPairs,
            (0, 1) => Kind::OnePair,
            _ => Kind::HighCard,
        }
    }

    pub fn kind2(&self) -> Kind {
        let replacements = "23456789TQKA".chars().map(Card::parse).flatten();
        let highest_kind_hand = replacements
            .map(|r| self.replace_joker(r))
            .max_by_key(|a| a.kind().rank())
            .unwrap();
        return highest_kind_hand.kind();
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let kind = self.kind();
        let other_kind = other.kind();

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

    fn cmp2(&self, other: &Self) -> std::cmp::Ordering {
        let kind = self.kind2();
        let other_kind = other.kind2();

        if kind.rank() == other_kind.rank() {
            for (card, other_card) in self.cards.iter().zip(&other.cards) {
                if card.value2() != other_card.value2() {
                    return card.value2().cmp(&&other_card.value2());
                }
            }
            return std::cmp::Ordering::Equal;
        } else {
            return kind.rank().cmp(&other_kind.rank());
        }
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

pub fn part1(input: String) {
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

    println!("{:?}", result);
}

pub fn part2(input: String) {
    let mut table = parse(&input);
    table.sort_by(|(a, _), (b, _)| a.cmp2(b));

    let result: usize = table
        .iter()
        .enumerate()
        .map(|(i, (h, b))| {
            println!("{i}: Hand {h:?} bet {b}");
            return b * (i + 1);
        })
        .sum();

    //248047349 too high
    println!("{:?}", result);
}

pub fn process(input: String) {
    part2(input);
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
        assert_eq!(Hand::parse("333AQ").kind(), Kind::ThreeOfAKind);
        assert_eq!(Hand::parse("23456").kind(), Kind::HighCard);
    }

    #[test]
    fn hand_order() {}
}
