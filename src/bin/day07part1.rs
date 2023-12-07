use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

use nom::character::complete::{alphanumeric1, digit1, space1};
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let input = include_str!("./inputs/day07");
    let res = solve(input);
    dbg!(res);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    TwoPairs,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_value(value: &str) -> Option<Self> {
        let mut map = HashMap::new();
        value.chars().for_each(|c| {
            if let Some(count) = map.get_mut(&c) {
                *count += 1;
            } else {
                map.insert(c, 1);
            }
        });

        let mut counts = map.values().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();

        match counts.len() {
            5 => Some(HandType::HighCard),
            1 => Some(HandType::Five),
            4 => Some(HandType::OnePair),
            2 => match counts.first().expect("Counts is empty") {
                4 => Some(HandType::Four),
                3 => Some(HandType::Full),
                _ => None,
            },
            3 => match counts.first().expect("Counts is empty") {
                3 => Some(HandType::Three),
                2 => Some(HandType::TwoPairs),
                _ => None,
            },
            _ => None,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            HandType::Five => match other {
                HandType::Five => Ordering::Equal,
                HandType::Four => Ordering::Greater,
                HandType::Full => Ordering::Greater,
                HandType::Three => Ordering::Greater,
                HandType::TwoPairs => Ordering::Greater,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::Four => match other {
                HandType::Five => Ordering::Less,
                HandType::Four => Ordering::Equal,
                HandType::Full => Ordering::Greater,
                HandType::Three => Ordering::Greater,
                HandType::TwoPairs => Ordering::Greater,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::Full => match other {
                HandType::Five => Ordering::Less,
                HandType::Four => Ordering::Less,
                HandType::Full => Ordering::Equal,
                HandType::Three => Ordering::Greater,
                HandType::TwoPairs => Ordering::Greater,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::Three => match other {
                HandType::Five => Ordering::Less,
                HandType::Four => Ordering::Less,
                HandType::Full => Ordering::Less,
                HandType::Three => Ordering::Equal,
                HandType::TwoPairs => Ordering::Greater,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::TwoPairs => match other {
                HandType::Five => Ordering::Less,
                HandType::Four => Ordering::Less,
                HandType::Full => Ordering::Less,
                HandType::Three => Ordering::Less,
                HandType::TwoPairs => Ordering::Equal,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::OnePair => match other {
                HandType::Five => Ordering::Less,
                HandType::Four => Ordering::Less,
                HandType::Full => Ordering::Less,
                HandType::Three => Ordering::Less,
                HandType::TwoPairs => Ordering::Less,
                HandType::OnePair => Ordering::Equal,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::HighCard => match other {
                HandType::Five => Ordering::Less,
                HandType::Four => Ordering::Less,
                HandType::Full => Ordering::Less,
                HandType::Three => Ordering::Less,
                HandType::TwoPairs => Ordering::Less,
                HandType::OnePair => Ordering::Less,
                HandType::HighCard => Ordering::Equal,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    pub value: String,
    pub bid: u32,
    pub hand_type: HandType,
}

impl Hand {
    pub fn new(value: &str, bid: u32) -> Self {
        Hand {
            value: value.to_owned(),
            bid,
            hand_type: HandType::from_value(value).expect("Could not identify the hand type"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                let cards = self.value.chars();
                let cards_other = other.value.chars();

                for (a, b) in zip(cards, cards_other) {
                    match a {
                        'A' => match b {
                            'A' => continue,
                            _ => return Ordering::Greater,
                        },
                        'K' => match b {
                            'A' => return Ordering::Less,
                            'K' => continue,
                            _ => return Ordering::Greater,
                        },
                        'Q' => match b {
                            'A' => return Ordering::Less,
                            'K' => return Ordering::Less,
                            'Q' => continue,
                            _ => return Ordering::Greater,
                        },
                        'J' => match b {
                            'A' => return Ordering::Less,
                            'K' => return Ordering::Less,
                            'Q' => return Ordering::Less,
                            'J' => continue,
                            _ => return Ordering::Greater,
                        },
                        'T' => match b {
                            'A' => return Ordering::Less,
                            'K' => return Ordering::Less,
                            'Q' => return Ordering::Less,
                            'J' => return Ordering::Less,
                            'T' => continue,
                            _ => return Ordering::Greater,
                        },
                        _ => {
                            if let Some(b) = b.to_digit(10) {
                                let a = a.to_digit(10).expect("Expected a numbered card");
                                match a.cmp(&b) {
                                    Ordering::Equal => continue,
                                    cmp => return cmp,
                                }
                            } else {
                                return Ordering::Less;
                            }
                        }
                    }
                }
                println!("out of loop");
                Ordering::Equal
            }
            x => x,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_race_duration(line: &str) -> IResult<&str, Hand> {
    let (i, (hand, _, bid)) = tuple((alphanumeric1, space1, digit1))(line)?;
    let bid = bid.parse::<u32>().expect("Could not parse bid");
    Ok((i, Hand::new(hand, bid)))
}

fn solve(input: &str) -> u32 {
    let mut hands = Vec::new();
    for line in input.lines() {
        let (_, hand) = parse_race_duration(line).expect("Could not parse hand");
        hands.push(hand);
    }
    hands.sort();
    dbg!(hands
        .clone()
        .iter()
        .filter(|h| h.hand_type == HandType::Four)
        .map(|h| h.value.clone())
        .collect::<Vec<_>>());
    let res = hands
        .iter()
        .map(|h| h.bid)
        .enumerate()
        .fold(0, |acc, (rank, bid)| acc + bid * (rank as u32 + 1));
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let res = solve(input);
        assert_eq!(res, 6440);
    }

    #[test]
    fn test_cmp() {
        let a = Hand::new("2222Q", 0);
        let b = Hand::new("22228", 0);
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }
}
