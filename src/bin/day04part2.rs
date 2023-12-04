use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::IResult;

fn main() {
    let input = include_str!("./inputs/day04");
    let res = solve(input);
    dbg!(res);
}

fn parse_card_number(line: &str) -> IResult<&str, u32> {
    let (line, _) = tag("Card")(line)?;
    let (line, _) = space1(line)?;
    let (line, card) = digit1(line)?;
    let card = card.parse::<u32>().expect("Could not cast to u32");
    let (line, _) = space0(line)?;
    Ok((line, card))
}

fn parse_single_number(line: &str) -> IResult<&str, u32> {
    let (line, _) = space0(line)?;
    let (line, number) = digit1(line)?;
    let number = number.parse::<u32>().expect("Could not cast to u32");
    let (line, _) = space0(line)?;
    Ok((line, number))
}

fn parse_numbers(line: &str) -> IResult<&str, Vec<u32>> {
    many0(parse_single_number)(line)
}

fn process_card(line: &str) -> usize {
    let (_, (_, (winning_numbers, numbers))) = separated_pair(
        parse_card_number,
        tag(":"),
        separated_pair(parse_numbers, tag("|"), parse_numbers),
    )(line)
    .expect("Could not parse card");

    let n_winners = numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count();
    n_winners
}

fn solve(input: &str) -> u32 {
    let input_size = input.lines().count();
    let mut card_amount = vec![1_u32; input_size];
    for (card, line) in input.lines().enumerate() {
        let n_winnings = process_card(line);
        let current_card_amount = card_amount[card];
        if n_winnings > 0 {
            for c in card_amount[card + 1..card + n_winnings + 1].as_mut() {
                *c += current_card_amount;
            }
        }
    }
    card_amount.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res = solve(input);
        assert_eq!(res, 30);
    }
}
