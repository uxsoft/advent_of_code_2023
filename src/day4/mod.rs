use std::collections::HashMap;

struct ScratchCard {
    id: u32,
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>,
}

impl ScratchCard {
    pub fn parse(line: &str) -> ScratchCard {
        let title_split = line.split(": ").collect::<Vec<_>>();

        let id: u32 = title_split
            .first()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let number_split = title_split.last().unwrap().split(" | ").collect::<Vec<_>>();
        let winning_numbers = number_split
            .first()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();

        let our_numbers = number_split
            .last()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();

        return ScratchCard {
            id,
            winning_numbers,
            our_numbers,
        };
    }

    pub fn matches(&self) -> usize {
        self.our_numbers
            .iter()
            .filter(|a| self.winning_numbers.contains(a))
            .count()
    }
}

pub fn part1(input: String) {
    let result: u32 = input
        .lines()
        .map(|line| {
            let card = ScratchCard::parse(line);

            let matches = card.matches();

            return if matches > 0 {
                2_u32.pow(matches as u32 - 1)
            } else {
                0
            };
        })
        .sum();

    println!("Result: {}", result);
}

pub fn process(input: String) {
    let cards: Vec<ScratchCard> = input.lines().map(|line| ScratchCard::parse(line)).collect();

    let mut card_count: HashMap<u32, u32> = cards.iter().map(|card| (card.id, 1)).collect();

    for card in cards {
        let wins = card.matches();
        println!("Card {} wins {}", card.id, wins);
        if wins > 0 {
            let duplicates = *card_count.get(&card.id).unwrap();

            for i in 1..=wins {
                println!("Adding {duplicates} duplicates of card {}", card.id + i as u32);
                card_count.entry(card.id + i as u32).and_modify(|a| *a += duplicates);
            }
        }
    }

    let result: u32 = card_count.values().sum();
    println!("Result: {}", result);
}
