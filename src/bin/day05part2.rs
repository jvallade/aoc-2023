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

fn parse_seeds(line: &str) -> IResult<&str, Vec<Range>> {
    let (i, _) = tag("seeds: ")(line)?;
    let (i, seeds_raw) = many0(tuple((space0, digit1, space0, digit1, space0)))(i)?;
    let seeds = seeds_raw
        .iter()
        .map(|(_, seed_start, _, seed_range, _)| {
            Range::new(
                seed_start.parse::<u64>().expect("Not a u64"),
                seed_range.parse::<u64>().expect("Not a u64"),
            )
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

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd)]
struct Range {
    pub start: u64,
    pub stop: u64,
    pub range: u64,
}

impl Range {
    fn new(start: u64, range: u64) -> Self {
        let stop = start + range - 1;
        Range { start, stop, range }
    }

    fn split(&self, map: &AlmanacMap) -> Vec<Range> {
        let mut split_values = Vec::new();
        split_values.push(self.start);
        for r in map.ranges.iter() {
            let stop = r.source_start + r.range - 1;
            if r.source_start > self.start && r.source_start < self.stop {
                split_values.push(r.source_start);
                split_values.push(r.source_start + 1);
            }
            if stop > self.start && stop < self.stop {
                split_values.push(stop);
                split_values.push(stop + 1);
            }
        }
        split_values.push(self.stop);
        split_values.sort();

        let mut res = Vec::new();
        let mut val_iter = split_values.iter();
        while let Some(start) = val_iter.next() {
            let stop = val_iter.next().expect("We expect to always have pairs");
            let range = stop + 1 - start;
            res.push(Range::new(*start, range));
        }
        res
    }

    fn convert(&self, map: &AlmanacMap) -> Self {
        let mut res = self.to_owned();
        if let Some(range) = map
            .ranges
            .iter()
            .find(|r| self.start >= r.source_start && (self.start < r.source_start + r.range))
        {
            res.start = range.destination_start + (self.start - range.source_start);
            res.stop = res.start + res.range - 1;
        }
        res
    }

    fn get_min_location(&self, maps: &HashMap<String, AlmanacMap>) -> u64 {
        let seed_map = maps.get("seed").expect("Seed map not found");
        let soil_map = maps.get("soil").expect("soil map not found");
        let fert_map = maps.get("fertilizer").expect("fertilizer map not found");
        let wate_map = maps.get("water").expect("water map not found");
        let ligh_map = maps.get("light").expect("light map not found");
        let temp_map = maps.get("temperature").expect("temperature map not found");
        let humi_map = maps.get("humidity").expect("humidity map not found");

        let ranges = self
            .split(seed_map)
            .iter()
            .map(|r| r.convert(seed_map))
            .collect::<Vec<_>>();

        let ranges = ranges
            .iter()
            .flat_map(|r| r.split(soil_map))
            .map(|r| r.convert(soil_map))
            .collect::<Vec<_>>();

        let ranges = ranges
            .iter()
            .flat_map(|r| r.split(fert_map))
            .map(|r| r.convert(fert_map))
            .collect::<Vec<_>>();

        let ranges = ranges
            .iter()
            .flat_map(|r| r.split(wate_map))
            .map(|r| r.convert(wate_map))
            .collect::<Vec<_>>();

        let ranges = ranges
            .iter()
            .flat_map(|r| r.split(ligh_map))
            .map(|r| r.convert(ligh_map))
            .collect::<Vec<_>>();

        let ranges = ranges
            .iter()
            .flat_map(|r| r.split(temp_map))
            .map(|r| r.convert(temp_map))
            .collect::<Vec<_>>();

        let ranges = ranges
            .iter()
            .flat_map(|r| r.split(humi_map))
            .map(|r| r.convert(humi_map))
            .collect::<Vec<_>>();

        ranges
            .iter()
            .map(|r| r.start)
            .min()
            .expect("Got empty loc list")
    }
}

fn solve(input: &str) -> u64 {
    let mut seeds: Vec<Range> = Vec::new();
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

    println!("Start seed populating");

    let res = dbg!(seeds)
        .iter()
        .map(|s| dbg!(s.get_min_location(&maps)))
        .min()
        .expect("seed list is empty");
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
        assert_eq!(res, 46);
    }
}
