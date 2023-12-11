use std::collections::BTreeSet;

fn main() {
    let input = include_str!("./inputs/day11");
    let res = solve(input);
    dbg!(res);
}

#[derive(Debug)]
struct Galaxy {
    x: u32,
    y: u32,
}

fn parse_map(input: &str) -> Vec<Galaxy> {
    let mut map = Vec::new();
    let mut offset = 0;
    for (y, line) in input.lines().enumerate() {
        if !line.contains('#') {
            offset += 1;
            continue;
        }

        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(x, _)| {
                map.push(Galaxy {
                    x: x as u32,
                    y: y as u32 + offset,
                })
            });
    }

    map.sort_by_key(|g| g.x);

    offset = 0;
    let galaxies_x = map.iter().map(|g| g.x as usize).collect::<BTreeSet<_>>();

    let mut empty_x = (0..input.lines().next().unwrap().len()).filter(|x| !galaxies_x.contains(x));
    if let Some(mut next_empty) = empty_x.next() {
        for galaxy in map.iter_mut() {
            if galaxy.x > next_empty as u32 {
                offset += 1;
                next_empty = match empty_x.next() {
                    Some(x) => x,
                    None => u64::MAX as usize,
                }
            }
            galaxy.x += offset;
        }
    }

    map
}

fn solve(input: &str) -> i64 {
    let map = parse_map(input);
    let mut res = 0;
    for i in 0..map.len() {
        for j in i..map.len() {
            if i != j {
                let g1 = map.get(i).expect("Could not retrieve galaxy");
                let g2 = map.get(j).expect("Could not retrieve galaxy");
                let dist = (g1.x as i64 - g2.x as i64).abs() + (g1.y as i64 - g2.y as i64).abs();
                res += dist;
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let res = solve(input);
        assert_eq!(res, 374);
    }
}
