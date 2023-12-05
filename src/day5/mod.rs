use rayon::prelude::*;
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
        let split: Vec<i64> = input
            .split(" ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Mapping {
            destination_start: *split.get(0).unwrap(),
            source_start: *split.get(1).unwrap(),
            length: *split.get(2).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Projection {
    mappings: Vec<Mapping>,
}

impl Projection {
    pub fn parse(input: &str) -> Projection {
        let mut mappings: Vec<Mapping> = input.lines().map(Mapping::parse).collect();
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
                .find(|m| i < m.source_start + m.length)
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

        println!("Mapped range {}-{} to {:?}", range.start, range.length, dest_ranges);
        return dest_ranges;
    }

    pub fn map_range_1(&self, range: &Range) -> Vec<Range> {
        let mut dest_ranges = vec![];

        let mut i = range.start;
        let mut l = range.length;

        for mapping in &self.mappings {
            if i < mapping.source_start {
                if i + l < mapping.source_start {
                    // We are before the next mapping and shorter than to get to it
                    // We return a range from i to l with no translation
                    // And we're done!
                    dest_ranges.push(Range::new(i, l));
                    return dest_ranges;
                } else {
                    // We are before the next mapping and will go beyond it's start
                    // Advance to mapping start
                    let distance_to_start = mapping.source_start - i;

                    dest_ranges.push(Range::new(i, distance_to_start));
                    i += distance_to_start;
                    l -= distance_to_start;

                    // Consume from the next mapping
                    let offset = i - mapping.source_start;
                    if i + l < mapping.source_start + mapping.length {
                        // Range ends inside of this mapping
                        // Push mapped range, return
                        dest_ranges.push(Range::new(mapping.destination_start + offset, l));
                        return dest_ranges;
                    } else {
                        // Range ends after this mapping
                        // Push mapped range until the end, continue;
                        let count = i + l - mapping.source_start - mapping.length;
                        dest_ranges.push(Range::new(mapping.destination_start + offset, count));
                        i += count;
                        l -= count;

                        continue;
                    }
                }
            } else {
                // i >= mapping.start
                if i > mapping.source_start + mapping.length {
                    // This mapping ends before our range starts
                    continue;
                } else if i + l < mapping.source_start + mapping.length {
                    // Range ends inside of this mapping
                    // Push mapped range, return
                    let offset = i - mapping.source_start;
                    dest_ranges.push(Range::new(mapping.destination_start + offset, l));
                    return dest_ranges;
                } else {
                    // Range ends after this mapping
                    // Push mapped range until the end, continue;
                    let offset = i - mapping.source_start;
                    let count = i + l - mapping.source_start - mapping.length;
                    dest_ranges.push(Range::new(mapping.destination_start + offset, count));
                    i += count;
                    l -= count;

                    continue;
                }
            }
        }

        return dest_ranges;
    }

    pub fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        ranges.iter().flat_map(|r| self.map_range(r)).collect()
    }
}

trait MappingChain {
    fn map_to(&self, source: i64) -> i64;
    fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range>;
}

impl MappingChain for Vec<&Projection> {
    fn map_to(&self, source: i64) -> i64 {
        let mut i = source;
        for m in self {
            i = m.map_to(i);
        }
        return i;
    }

    fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut i = ranges;
        for m in self {
            i = m.map_ranges(i);
        }
        return i;
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: Projection,
    soil_to_fertilizer: Projection,
    fertilizer_to_water: Projection,
    water_to_light: Projection,
    light_to_temperature: Projection,
    temperature_to_humidity: Projection,
    humidity_to_location: Projection,
}

impl Almanac {
    pub fn parse(input: String) -> Almanac {
        let regex = Regex::new(r"seeds: ([\s\d]+)\r\n\r\nseed-to-soil map:\r\n([\s\d]+)\r\n\r\nsoil-to-fertilizer map:\r\n([\s\d]+)\r\n\r\nfertilizer-to-water map:\r\n([\s\d]+)\r\n\r\nwater-to-light map:\r\n([\s\d]+)\r\n\r\nlight-to-temperature map:\r\n([\s\d]+)\r\n\r\ntemperature-to-humidity map:\r\n([\s\d]+)\r\n\r\nhumidity-to-location map:\r\n([\s\d]+)").unwrap();
        let captures = regex.captures(&input).unwrap();

        Almanac {
            seeds: captures
                .get(1)
                .unwrap()
                .as_str()
                .split(" ")
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
            seed_to_soil: Projection::parse(captures.get(2).unwrap().as_str()),
            soil_to_fertilizer: Projection::parse(captures.get(3).unwrap().as_str()),
            fertilizer_to_water: Projection::parse(captures.get(4).unwrap().as_str()),
            water_to_light: Projection::parse(captures.get(5).unwrap().as_str()),
            light_to_temperature: Projection::parse(captures.get(6).unwrap().as_str()),
            temperature_to_humidity: Projection::parse(captures.get(7).unwrap().as_str()),
            humidity_to_location: Projection::parse(captures.get(8).unwrap().as_str()),
        }
    }

    pub fn get_seed_to_location_chain(&self) -> Vec<&Projection> {
        vec![
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
    }
}

pub fn part1(input: String) {
    let almanac = Almanac::parse(input);
    let chain = almanac.get_seed_to_location_chain();

    let result = almanac
        .seeds
        .iter()
        .map(|s| chain.map_to(*s))
        .min()
        .unwrap();

    println!("Result: {:?}", result);
}

pub fn part2(input: String) {
    let almanac = Almanac::parse(input);
    let chain = almanac.get_seed_to_location_chain();

    let seed_ranges: Vec<Range> = almanac
        .seeds
        .chunks(2)
        .map(|w| Range {
            start: w[0],
            length: w[1],
        })
        .take(1) // Take this out for a sharp run
        .collect();

    let result = chain.map_ranges(seed_ranges).iter().map(|r| r.start).min();

    println!("Result: {:?}", result);
}

pub fn part2_bf(input: String) {
    let almanac = Almanac::parse(input);
    let chain = almanac.get_seed_to_location_chain();

    let seeds: Vec<i64> = almanac
        .seeds
        .chunks(2)
        .flat_map(|w| w[0]..(w[0] + w[1]))
        .collect();

    let result = seeds.iter().map(|s| chain.map_to(*s)).min().unwrap();

    println!("Result: {:?}", result);
}

pub fn process(input: String) {
    part2_bf(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soil_mappings() {
        let almanac = Almanac::parse(include_str!("example.txt").to_string());
        assert_eq!(almanac.seed_to_soil.map_to(79), 81);
        assert_eq!(almanac.seed_to_soil.map_to(14), 14);
        assert_eq!(almanac.seed_to_soil.map_to(55), 57);
        assert_eq!(almanac.seed_to_soil.map_to(13), 13);
    }

    #[test]
    fn location_mappings() {
        let almanac = Almanac::parse(include_str!("example.txt").to_string());
        let chain = almanac.get_seed_to_location_chain();

        assert_eq!(chain.map_to(79), 82);
        assert_eq!(chain.map_to(14), 43);
        assert_eq!(chain.map_to(55), 86);
        assert_eq!(chain.map_to(13), 35);
    }

    #[test]
    fn part2_solution() {
        let almanac = Almanac::parse(include_str!("example.txt").to_string());
        let chain = almanac.get_seed_to_location_chain();

        assert_eq!(chain.map_to(82), 46);
    }
}
