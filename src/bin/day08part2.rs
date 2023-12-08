use std::collections::BTreeMap;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let input = include_str!("./inputs/day08");
    let res = solve(input);
    dbg!(res);
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(name: &str, left: &str, right: &str) -> Self {
        Node {
            name: name.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }

    fn get_next(&self, instruction: &Instruction) -> &str {
        match instruction {
            Instruction::Left => &self.left,
            Instruction::Right => &self.right,
        }
    }
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    line.chars()
        .map(|c| match c {
            'L' => Some(Instruction::Left),
            'R' => Some(Instruction::Right),
            _ => None,
        })
        .map(|c| c.expect("Unexpected instruction"))
        .collect()
}

fn parse_node(line: &str) -> IResult<&str, Node> {
    let (i, (name, _, left, _, right)) =
        tuple((alpha1, tag(" = ("), alpha1, tag(", "), alpha1))(line)?;
    Ok((i, Node::new(name, left, right)))
}

fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = parse_instructions(lines.next().expect("instruction line not found"));
    lines.next();

    let mut nodes = BTreeMap::new();
    for line in lines {
        let (_, node) = parse_node(line).expect("Could not parse node");
        nodes.insert(node.name.to_owned(), node);
    }

    let mut res = Vec::new();

    let starting_nodes = nodes
        .values()
        .filter(|n| n.name.ends_with('A'))
        .collect::<Vec<_>>();
    for starting_node in starting_nodes {
        let mut current_node = starting_node;
        for (index, instruction) in instructions.iter().cycle().enumerate() {
            let next_name = current_node.get_next(instruction);
            current_node = nodes.get(next_name).expect("Unknown node");

            if next_name.ends_with('Z') {
                dbg!(&current_node.name);
                res.push(dbg!(index + 1));
                break;
            }
        }
    }
    // lcm(res.iter().fold(1, |acc, l| num::integer::lcm(acc, *l))
    lcm(&res)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let res = solve(input);
        assert_eq!(res, 2);
    }
    #[test]
    fn it_works2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let res = solve(input);
        assert_eq!(res, 6);
    }
}
