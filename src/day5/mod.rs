use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Range {
    start: i64,
    length: i64,
}

impl Range {
    pub fn new(start: i64, length: i64) -> Self {
        Self { start, length }
    }
}

#[derive(Debug)]
struct Mapping {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

impl Mapping {
    pub fn parse(input: &str) -> Mapping {
        let split: Vec<i64> = input.split(" ").map(str::parse).flatten().collect();

        Mapping {
            destination_start: *split.get(0).unwrap(),
            source_start: *split.get(1).unwrap(),
            length: *split.get(2).unwrap(),
        }
    }

    fn destination_end(&self) -> i64 {
        self.destination_start + self.length - 1
    }

    fn source_end(&self) -> i64 {
        self.source_start + self.length - 1
    }
}

#[derive(Debug)]
struct Projection {
    mappings: Vec<Mapping>,
}

impl Projection {
    pub fn parse(input: &str) -> Projection {
        let mut mappings: Vec<Mapping> = input.lines().skip(1).map(Mapping::parse).collect();
        mappings.sort_by(|a, b| a.source_start.partial_cmp(&b.source_start).unwrap());

        Projection { mappings }
    }

    pub fn map_to(&self, source: i64) -> i64 {
        for mapping in &self.mappings {
            if source >= mapping.source_start && source < mapping.source_start + mapping.length {
                return source + (mapping.destination_start - mapping.source_start);
            }
        }
        return source;
    }

    pub fn map_range(&self, range: &Range) -> Vec<Range> {
        let mut dest_ranges = vec![];

        let mut i = range.start;
        let mut l = range.length;

        while l > 0 {
            println!("l: {l}");

            // Pass mappings where i > mapping.start + mapping.length (they end before this range starts)
            if let Some(mapping) = self
                .mappings
                .iter()
                .find(|m| i < m.source_start + m.length - 1)
            {
                if i >= mapping.source_start {
                    // We are inside of the mapping, advance to the end of the mapping or as long as `l` goes
                    let count = l.min(mapping.source_start + mapping.length - i);
                    let offset = i - mapping.source_start;
                    dest_ranges.push(Range::new(mapping.destination_start + offset, count));
                    i += count;
                    l -= count;
                } else {
                    // We are before the mapping but `l` ends in or after it, advance to the start of the mapping with an isomorphic projection
                    let count = mapping.source_start - i;
                    dest_ranges.push(Range::new(i, count));
                    i += count;
                    l -= count;
                }
            } else {
                // Passed all the mappings, apply an isomorphic projection
                dest_ranges.push(Range::new(i, l));
                break;
            }
        }

        println!(
            "Mapped range {}-{} to {:?}",
            range.start, range.length, dest_ranges
        );
        return dest_ranges;
    }

    pub fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        ranges.iter().flat_map(|r| self.map_range(r)).collect()
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    projections: Vec<Projection>,
}

impl Almanac {
    pub fn parse(input: &str) -> Self {
        let double_line_ending: Regex = Regex::new(r"\r?\n\r?\n").unwrap();

        let sections = double_line_ending.split(input).collect_vec();

        let seeds: Vec<i64> = sections
            .first()
            .unwrap()
            .trim_start_matches("seeds: ")
            .split(" ")
            .map(str::parse)
            .flatten()
            .collect_vec();

        let projections = sections
            .iter()
            .skip(1)
            .map(|section: &&str| Projection::parse(&section))
            .collect_vec();

        Self { seeds, projections }
    }

    fn map_to(&self, source: i64) -> i64 {
        let mut i = source;
        for m in &self.projections {
            i = m.map_to(i);
        }
        return i;
    }

    fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut i = ranges;
        for m in &self.projections {
            i = m.map_ranges(i);
        }
        return i;
    }
}

pub fn part1(input: &str) -> i64 {
    let almanac = Almanac::parse(input);

    let result = almanac
        .seeds
        .iter()
        .map(|s| almanac.map_to(*s))
        .min()
        .unwrap();

    return result;
}

pub fn part2(input: &str) -> i64 {
    let almanac = Almanac::parse(input);

    let seed_ranges: Vec<Range> = almanac
        .seeds
        .chunks(2)
        .map(|w| Range {
            start: w[0],
            length: w[1],
        })
        .take(1) // Take this out for a sharp run
        .collect();

    let result = almanac
        .map_ranges(seed_ranges)
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap();

    return result;
}

pub fn part2_bf(input: &str) -> i64 {
    let almanac = Almanac::parse(input);

    let seeds: Vec<i64> = almanac
        .seeds
        .chunks(2)
        .flat_map(|w| w[0]..(w[0] + w[1]))
        .collect();

    let result = seeds.iter().map(|s| almanac.map_to(*s)).min().unwrap();

    return result;
}

pub fn process(input: String) {
    let result = part2(&input);
    println!("Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn soil_mappings() {
        let almanac = Almanac::parse(EXAMPLE);
        assert_eq!(almanac.projections.first().unwrap().map_to(79), 81);
        assert_eq!(almanac.projections.first().unwrap().map_to(14), 14);
        assert_eq!(almanac.projections.first().unwrap().map_to(55), 57);
        assert_eq!(almanac.projections.first().unwrap().map_to(13), 13);
    }

    #[test]
    fn location_mappings() {
        let almanac = Almanac::parse(EXAMPLE);

        assert_eq!(almanac.map_to(79), 82);
        assert_eq!(almanac.map_to(14), 43);
        assert_eq!(almanac.map_to(55), 86);
        assert_eq!(almanac.map_to(13), 35);
    }

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 35);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 579439039);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 46);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 7873084);
    }
}
