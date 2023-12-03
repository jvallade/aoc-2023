use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, space0, space1},
    multi::{many0, many1},
    IResult, Parser,
};

const RADIX: u32 = 10;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("./inputs/day02");
    let res = part1(input);
    dbg!(res);
}

#[derive(Debug)]
struct CubeCount {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeCount {
    fn new(input: Vec<(&str, &str)>) -> Self {
        let mut cc = CubeCount {
            red: 0,
            green: 0,
            blue: 0,
        };
        input.iter().for_each(|(count, color)| match *color {
            "red" => cc.red = count.parse().expect("could not parse count"),
            "green" => cc.green = count.parse().expect("could not parse count"),
            "blue" => cc.blue = count.parse().expect("could not parse count"),
            _ => {}
        });
        cc
    }
}

fn parse_game_id(input: &str) -> IResult<&str, &str> {
    let (i, _) = tag("Game ").parse(input)?;
    let (i, game_id) = take_while(|c: char| c.is_digit(RADIX))(i)?;
    Ok((i, game_id))
}

fn parse_color(input: &str) -> IResult<&str, (&str, &str)> {
    let (i, _) = space0(input)?;
    let (i, number) = take_while(|c: char| c.is_digit(RADIX))(i)?;
    let (i, _) = space1(i)?;
    let (i, color) = alt((tag("red"), tag("blue"), tag("green")))(i)?;
    let (i, _) = many0(char(','))(i)?;
    Ok((i, (number, color)))
}

fn parse_cubes(input: &str) -> IResult<&str, CubeCount> {
    let mut cubes_parser = many1(parse_color);
    let (i, colors) = cubes_parser(input)?;
    Ok((i, CubeCount::new(colors)))
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    for line in input.lines() {
        let mut parts = line.split(':');
        let id_part = parts.next().expect("Could not find the ID part");
        let (_, game_id) = parse_game_id(id_part).expect("Could not extract game ID");
        let game_id = game_id.parse::<u32>().expect("Could not parse game ID");

        let cubes_part = parts.next().expect("Could not find the cubes part");
        let mut possible = true;
        for cubes_input in cubes_part.split(';') {
            let (_, cc) = parse_cubes(cubes_input).expect("Could not parse cubes");
            if cc.red > MAX_RED {
                possible &= false;
            }
            if cc.green > MAX_GREEN {
                possible &= false;
            }
            if cc.blue > MAX_BLUE {
                possible &= false;
            }
        }
        if possible {
            res += game_id;
        }
    }
    res
}
