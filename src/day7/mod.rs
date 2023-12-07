use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{map, map_res, value},
    multi::{many1, separated_list1},
    number,
    sequence::{self, separated_pair},
    IResult, Parser,
};

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

    pub fn parse(input: &str) -> IResult<&str, Card> {
        let number = map_res(digit1, str::parse);

        let mut parser = alt((
            value(Card::A, tag("A")),
            value(Card::K, tag("K")),
            value(Card::Q, tag("Q")),
            value(Card::J, tag("J")),
            value(Card::T, tag("T")),
            map(number, Card::N),
        ));

        parser(input)
    }
}

fn parse(input: &str) -> Vec<(Vec<Card>, u64)> {
    let number = map_res(digit1, str::parse);

    let mut parser = separated_list1(
        line_ending,
        separated_pair(many1(Card::parse), space1, number),
    );

    let (_, result) = parser.parse(input).unwrap();
    return result;
}

pub fn process(input: String) {
    let table = parse(&input);

    println!("{:?}", table);
}
