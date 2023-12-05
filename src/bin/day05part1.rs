use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space0, space1};
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{IResult, Parser};

fn main() {
    let input = include_str!("./inputs/day05");
    let res = solve(input);
    dbg!(res);
}

fn parse_seeds(line: &str) -> IResult<&str, Vec<Seed>> {
    let (i, _) = tag("seeds: ")(line)?;
    let (i, seeds_raw) = many0(tuple((space0, digit1, space0)))(i)?;
    let seeds = seeds_raw
        .iter()
        .map(|(_, seed, _)| Seed {
            seed: seed.parse::<u64>().expect("Not a u64"),
            ..Default::default()
        })
        .collect();
    Ok((i, seeds))
}

fn parse_map_name(line: &str) -> IResult<&str, (&str, &str)> {
    tuple((alpha1, tag("-to-"), alpha1))
        .map(|(source, _, dest)| (source, dest))
        .parse(line)
}

fn parse_map_content(line: &str) -> IResult<&str, (u64, u64, u64)> {
    tuple((
        map_res(digit1, str::parse::<u64>),
        space1,
        map_res(digit1, str::parse::<u64>),
        space1,
        map_res(digit1, str::parse::<u64>),
    ))
    .map(|(dest, _, source, _, range)| (dest, source, range))
    .parse(line)
}

#[derive(Debug)]
struct AlmanacRange {
    pub source_start: u64,
    pub destination_start: u64,
    pub range: u64,
}

impl AlmanacRange {
    fn new(source_start: u64, destination_start: u64, range: u64) -> Self {
        AlmanacRange {
            source_start,
            destination_start,
            range,
        }
    }
}

#[derive(Debug)]
struct AlmanacMap {
    pub source: String,
    pub _destination: String,
    pub ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    fn new(source: &str, dest: &str) -> Self {
        AlmanacMap {
            source: source.into(),
            _destination: dest.into(),
            ranges: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
struct Seed {
    pub seed: u64,
    pub soil: u64,
    pub fertilizer: u64,
    pub water: u64,
    pub light: u64,
    pub temperature: u64,
    pub humidity: u64,
    pub location: u64,
}

fn convert(value: u64, map: &AlmanacMap) -> u64 {
    if let Some(range) = map
        .ranges
        .iter()
        .find(|r| value >= r.source_start && (value <= r.source_start + r.range))
    {
        range.destination_start + (value - range.source_start)
    } else {
        value
    }
}

impl Seed {
    fn populate(&mut self, maps: &HashMap<String, AlmanacMap>) {
        self.soil = convert(self.seed, maps.get("seed").expect("Seed map not found"));
        self.fertilizer = convert(self.soil, maps.get("soil").expect("Soil map not found"));
        self.water = convert(
            self.fertilizer,
            maps.get("fertilizer").expect("Fertilizer map not found"),
        );
        self.light = convert(self.water, maps.get("water").expect("Water map not found"));
        self.temperature = convert(self.light, maps.get("light").expect("Light map not found"));
        self.humidity = convert(
            self.temperature,
            maps.get("temperature").expect("Temperature map not found"),
        );
        self.location = convert(
            self.humidity,
            maps.get("humidity").expect("Humidity map not found"),
        );
    }
}

fn solve(input: &str) -> u64 {
    let mut seeds: Vec<Seed> = Vec::new();
    let mut maps: Vec<AlmanacMap> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        } else if let Ok((_, (dest, source, range))) = parse_map_content(line) {
            maps.last_mut()
                .expect("No map yet !")
                .ranges
                .push(AlmanacRange::new(source, dest, range))
        } else if let Ok((_, (source, dest))) = parse_map_name(line) {
            maps.push(AlmanacMap::new(source, dest));
        } else if let Ok((_, seeds_parsed)) = parse_seeds(line) {
            seeds = seeds_parsed;
        }
    }
    let maps = maps
        .into_iter()
        .map(|map| (map.source.clone(), map))
        .collect::<HashMap<String, AlmanacMap>>();
    seeds.iter_mut().for_each(|s| s.populate(&maps));
    let res = seeds
        .iter()
        .map(|s| s.location)
        .min()
        .expect("seed list is empty");
    dbg!(seeds);
    // dbg!(maps);
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "seeds: 79 14 55 13

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
        let res = solve(input);
        assert_eq!(res, 35);
    }
}
