use rayon::prelude::*;
use regex::Regex;

struct Range {
    start: i64,
    length: i64,
}

#[derive(Debug)]
struct MappingRange {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl MappingRange {
    pub fn parse(input: &str) -> MappingRange {
        let split: Vec<i64> = input
            .split(" ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        MappingRange {
            destination_range_start: *split.get(0).unwrap(),
            source_range_start: *split.get(1).unwrap(),
            range_length: *split.get(2).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    // pub fn new() -> Mapping {
    //     Mapping {
    //         ranges: vec![MappingRange {}],
    //     }
    // }

    pub fn parse(input: &str) -> Mapping {
        Mapping {
            ranges: input.lines().map(MappingRange::parse).collect(),
        }
    }

    pub fn map_to(&self, source: i64) -> i64 {
        for range in &self.ranges {
            if source >= range.source_range_start
                && source < range.source_range_start + range.range_length
            {
                return source + (range.destination_range_start - range.source_range_start);
            }
        }
        return source;
    }

    pub fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut destinations = vec![];

        for range in ranges {
            let mut i = range.start


        }

        return destinations;
    }
}

trait MappingChain {
    fn map_to(&self, source: i64) -> i64;
    fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range>;
}

impl MappingChain for Vec<&Mapping> {
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
    seed_to_soil: Mapping,
    soil_to_fertilizer: Mapping,
    fertilizer_to_water: Mapping,
    water_to_light: Mapping,
    light_to_temperature: Mapping,
    temperature_to_humidity: Mapping,
    humidity_to_location: Mapping,
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
            seed_to_soil: Mapping::parse(captures.get(2).unwrap().as_str()),
            soil_to_fertilizer: Mapping::parse(captures.get(3).unwrap().as_str()),
            fertilizer_to_water: Mapping::parse(captures.get(4).unwrap().as_str()),
            water_to_light: Mapping::parse(captures.get(5).unwrap().as_str()),
            light_to_temperature: Mapping::parse(captures.get(6).unwrap().as_str()),
            temperature_to_humidity: Mapping::parse(captures.get(7).unwrap().as_str()),
            humidity_to_location: Mapping::parse(captures.get(8).unwrap().as_str()),
        }
    }

    pub fn get_seed_to_location_chain(&self) -> Vec<&Mapping> {
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

    let seeds: Vec<i64> = almanac
        .seeds
        .chunks(2)
        .map(|w| Range {
            start: w[0],
            length: w[1],
        })
        .collect();
    println!("Seeds: {:?}", seeds);

    let result = seeds.par_iter().map(|s| chain.map_to(*s)).min().unwrap();

    println!("Result: {:?}", result);
}

pub fn process(input: String) {
    part2(input);
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
}
