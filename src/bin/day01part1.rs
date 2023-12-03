fn main() {
    let input = include_str!("./inputs/day01");
    let res = part1(input);
    dbg!(res);
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    for line in input.lines() {
        res += extract_digits(line);
    }
    res
}

fn extract_digits(line: &str) -> u32 {
    let mut caps = Vec::new();
    for char in line.chars() {
        if char.is_ascii_digit() {
            caps.push(char);
        }
    }
    caps.first().unwrap().to_digit(10).unwrap() * 10 + caps.last().unwrap().to_digit(10).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "toto1titit";
        let res = extract_digits(input);
        assert_eq!(res, 11);
    }
}
