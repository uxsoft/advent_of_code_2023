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

    pub fn matches(&self) -> u32 {
        let matches = self
            .our_numbers
            .iter()
            .filter(|n| self.winning_numbers.iter().any(|w| *w == **n))
            .count();

        return matches;
    }
}

pub fn process(input: String) {
    let result: u32 = input
        .lines()
        .map(|line| {
            let card = ScratchCard::parse(line);

            println!("{}: {matches}", title_split.first().unwrap());
            return if matches > 0 {
                2_u32.pow(matches as u32 - 1)
            } else {
                0
            };
        })
        .sum();

    println!("Result: {}", result);
}
