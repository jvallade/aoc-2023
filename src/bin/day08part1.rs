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

fn solve(input: &str) -> u32 {
    let mut lines = input.lines();
    let instructions = parse_instructions(lines.next().expect("instruction line not found"));
    lines.next();

    let mut nodes = BTreeMap::new();
    for line in lines {
        let (_, node) = parse_node(line).expect("Could not parse node");
        nodes.insert(node.name.to_owned(), node);
    }

    let mut step_count = 0;
    let mut instructions_iter = instructions.iter();
    let mut current_node = nodes.get("AAA").expect("Could not find starting node");
    loop {
        step_count += 1;
        dbg!(current_node);
        let instruction = match instructions_iter.next() {
            Some(i) => i,
            None => {
                instructions_iter = instructions.iter();
                instructions_iter
                    .next()
                    .expect("We expected the first instruction again")
            }
        };
        let next_name = current_node.get_next(instruction);
        current_node = nodes.get(next_name).expect("Unknown node");
        if current_node.name == "ZZZ" {
            break;
        }
    }
    step_count
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
