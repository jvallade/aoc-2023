use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, space0, space1},
    multi::{many0, many1},
    IResult,
};

const RADIX: u32 = 10;

fn main() {
    let input = include_str!("./inputs/day02");
    let res = part2(input);
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

    fn get_power(&self) -> u32 {
        self.red * self.green * self.blue
    }
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

fn part2(input: &str) -> u32 {
    let mut res = 0;
    for line in input.lines() {
        let mut parts = line.split(':');
        let _ = parts.next().expect("Could not find the ID part");

        let cubes_part = parts.next().expect("Could not find the cubes part");
        let mut cc_max = CubeCount::new(Vec::new());
        for cubes_input in cubes_part.split(';') {
            let (_, cc) = parse_cubes(cubes_input).expect("Could not parse cubes");
            if cc.red > cc_max.red {
                cc_max.red = cc.red;
            }
            if cc.green > cc_max.green {
                cc_max.green = cc.green;
            }
            if cc.blue > cc_max.blue {
                cc_max.blue = cc.blue;
            }
        }
        res += cc_max.get_power();
    }
    res
}
