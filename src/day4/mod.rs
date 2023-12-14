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

pub fn part1(input: &str) -> u32 {
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

    return result;
}

pub fn part2(input: &str) -> u32 {
    let cards: Vec<ScratchCard> = input.lines().map(|line| ScratchCard::parse(line)).collect();

    let mut card_count: HashMap<u32, u32> = cards.iter().map(|card| (card.id, 1)).collect();

    for card in cards {
        let wins = card.matches();

        if wins > 0 {
            let duplicates = *card_count.get(&card.id).unwrap();

            for i in 1..=wins {
                card_count
                    .entry(card.id + i as u32)
                    .and_modify(|a| *a += duplicates);
            }
        }
    }

    let result: u32 = card_count.values().sum();
    return result;
}

pub fn process(input: String) {
    let result = part2(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 19855);
    }

    #[test]
    fn part2_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(input);
        assert_eq!(result, 30);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 10378710);
    }
}
