fn main() {
    let input = include_str!("./inputs/day01");
    let res = part1(input);
    dbg!(res);
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    for line in input.lines() {
        res += dbg!(extract_digits(line));
    }
    res
}

fn replace_spelled_digits(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ee")
        .replace("four", "f4ur")
        .replace("five", "f5ve")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "n9ne")
}

fn extract_digits(line: &str) -> u32 {
    let mut caps = Vec::new();
    let clean_line = replace_spelled_digits(dbg!(line));
    for char in dbg!(clean_line).chars() {
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
    #[test]
    fn test_spelled() {
        let input = "two3four";
        let res = extract_digits(input);
        assert_eq!(res, 24);
    }
}
