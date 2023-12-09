use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let input = include_str!("./inputs/day09");
    let res = solve(input);
    dbg!(res);
}

fn parse_history(line: &str) -> IResult<&str, Vec<i64>> {
    let (i, values) = many1(tuple((space0, many0(tag("-")), digit1, space0)))(line)?;
    let values = values
        .iter()
        .map(|(_, sign, value, _)| {
            if sign.is_empty() {
                value.parse::<i64>().expect("Could not parse i64")
            } else {
                -value.parse::<i64>().expect("Could not parse i64")
            }
        })
        .collect::<Vec<_>>();

    Ok((i, values))
}

fn extrapolate(history: &[i64]) -> i64 {
    let mut intervals = Vec::new();
    let mut interval = history.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    intervals.push(interval.clone());
    loop {
        interval = interval.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        if interval.iter().all(|v| *v == 0) {
            break;
        }
        intervals.push(interval.clone());
    }
    let mut next_interval = 0;
    while let Some(interval) = intervals.pop() {
        next_interval += interval.last().expect("Should not be empty");
    }
    history.last().expect("Should not be empty") + next_interval
}

fn solve(input: &str) -> i64 {
    let mut res = 0;
    for line in input.lines() {
        let (_, history) = parse_history(line).expect("Could not parse input line");
        res += dbg!(extrapolate(&history));
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let res = solve(input);
        assert_eq!(res, 114);
    }
}
