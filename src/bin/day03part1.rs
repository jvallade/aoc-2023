use nom::bytes::complete::{take_till, take_while};
use nom::character::complete::anychar;
use nom::character::is_digit;
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::Err;
use nom::IResult;

fn main() {
    let input = include_str!("./inputs/day03");
    let res = part1(input);
    dbg!(res);
}

fn next_engine_part_number(line: &str) -> IResult<&str, (usize, &str)> {
    if line.is_empty() {
        return Err(Err::Error(Error::new(line, ErrorKind::Complete)));
    }
    let (line, x) = take_till(|c: char| is_digit(c as u8))(line)?;
    let (line, part_number) = take_while(|c: char| is_digit(c as u8))(line)?;
    Ok((line, (x.len(), part_number)))
}

fn engine_part_number_parser(line: &str, line_number: usize) -> Vec<EnginePartNumber> {
    let (_, entries) =
        many0(next_engine_part_number)(line).expect("Could not parse engine part numbers");
    let mut current_x = 0;
    let mut res = Vec::new();
    for entry in entries {
        if !entry.1.is_empty() {
            current_x += entry.0;
            res.push(EnginePartNumber::new(
                current_x as i64,
                line_number as i64,
                entry.1,
            ));
        }
        current_x += entry.1.len();
    }
    res
}

fn is_not_engine_part(c: char) -> bool {
    is_digit(c as u8) || c == '.'
}

fn next_engine_part(line: &str) -> IResult<&str, (usize, char)> {
    if line.is_empty() {
        return Err(Err::Error(Error::new(line, ErrorKind::Complete)));
    }
    let (line, x) = take_while(is_not_engine_part)(line)?;
    let (line, symbol) = anychar(line)?;
    Ok((line, (x.len(), symbol)))
}

fn engine_part_parser(line: &str, line_number: usize) -> Vec<EnginePart> {
    let (_, entries) = many0(next_engine_part)(line).expect("Could not parse engine parsts");
    let mut current_x = 0;
    let mut res = Vec::new();
    for entry in entries {
        current_x += entry.0;
        res.push(EnginePart::new(
            current_x as i64,
            line_number as i64,
            entry.1,
        ));
        current_x += 1;
    }
    res
}

fn is_near_engine_part(number: &EnginePartNumber, parts: &[EnginePart]) -> bool {
    let res = parts.iter().any(|p| {
        if p.y == number.y {
            p.x == number.x_min - 1 || p.x == number.x_max + 1
        } else if p.y == number.y - 1 || p.y == number.y + 1 {
            p.x >= number.x_min - 1 && p.x <= number.x_max + 1
        } else {
            false
        }
    });
    res
}

fn part1(input: &str) -> u32 {
    let mut engine_part_numbers = Vec::new();
    let mut engine_parts = Vec::new();
    for (y, line) in input.lines().enumerate() {
        engine_part_numbers.extend(engine_part_number_parser(line, y));
        engine_parts.extend(engine_part_parser(line, y));
    }
    engine_part_numbers
        .iter()
        .filter(|n| is_near_engine_part(n, &engine_parts))
        .map(|n| n.value)
        .sum()
}

#[derive(Debug)]
pub struct EnginePart {
    pub x: i64,
    pub y: i64,
    pub value: char,
}

impl EnginePart {
    pub fn new(x: i64, y: i64, value: char) -> Self {
        EnginePart { x, y, value }
    }
}

#[derive(Debug, Default)]
pub struct EnginePartNumber {
    pub x_min: i64,
    pub x_max: i64,
    pub y: i64,
    pub value: u32,
}

impl EnginePartNumber {
    pub fn new(x: i64, y: i64, value: &str) -> Self {
        let width = value.len() as i64;
        let value = value.parse::<u32>().expect("Value is not an uint32");
        EnginePartNumber {
            x_min: x,
            x_max: x + width - 1,
            y,
            value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let res = part1(input);
        assert_eq!(res, 4361);
    }

    #[test]
    fn it_works2() {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";
        let res = part1(input);
        assert_eq!(res, 413);
    }
}
