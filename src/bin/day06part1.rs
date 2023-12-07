use std::iter::zip;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let input = include_str!("./inputs/day06");
    let res = solve(input);
    dbg!(res);
}

fn parse_races_duration(line: &str) -> IResult<&str, Vec<u32>> {
    let (i, _) = tuple((tag("Time:"), space0))(line)?;
    let (i, durations) = many0(tuple((space0, digit1, space0)))(i)?;
    let durations = durations
        .iter()
        .map(|(_, duration, _)| duration.parse::<u32>().expect("Not a u32"))
        .collect();

    Ok((i, durations))
}

fn parse_races_distances(line: &str) -> IResult<&str, Vec<u32>> {
    let (i, _) = tuple((tag("Distance:"), space0))(line)?;
    let (i, distances) = many0(tuple((space0, digit1, space0)))(i)?;
    let distances = distances
        .iter()
        .map(|(_, distance, _)| distance.parse::<u32>().expect("Not a u32"))
        .collect();

    Ok((i, distances))
}

fn roots(duration: f64, distance: f64) -> (f64, f64) {
    let delta = duration.powf(2.0) - 4.0 * distance;
    let root1 = (duration + delta.sqrt()) / 2.0;
    let root2 = (duration - delta.sqrt()) / 2.0;
    (root2, root1)
}

fn compute_distance(push_duration: f64, race_duration: f64) -> f64 {
    push_duration * (race_duration - push_duration)
}

fn solve(input: &str) -> usize {
    let mut input = input.lines();
    let (_, durations) = parse_races_duration(input.next().expect("Duration line not found"))
        .expect("Could not parse durations");
    let (_, distances) = parse_races_distances(input.next().expect("Distance line not found"))
        .expect("Could not parse distances");
    let mut solutions = Vec::new();
    for (duration, distance) in zip(durations.iter(), distances.iter()) {
        let (t_min, t_max) = roots(*duration as f64, *distance as f64);
        let mut t_min = t_min.ceil() as u32;
        let mut t_max = t_max.floor() as u32;
        if compute_distance(t_min as f64, *duration as f64) <= *distance as f64 {
            t_min += 1;
        }
        if compute_distance(t_max as f64, *duration as f64) <= *distance as f64 {
            t_max -= 1;
        }
        solutions.push((t_min..t_max).len() + 1);
    }
    solutions.iter().product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let res = solve(input);
        assert_eq!(res, 288);
    }
}
