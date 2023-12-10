fn main() {
    let input = include_str!("./inputs/day10");
    let res = solve(input);
    dbg!(res);
}

#[derive(Debug, PartialEq, Clone)]
enum Pipe {
    Vertical(usize, usize),
    Horizontal(usize, usize),
    BendNE(usize, usize),
    BendNW(usize, usize),
    BendSW(usize, usize),
    BendSE(usize, usize),
    Ground(usize, usize),
    Start(usize, usize),
}

impl Pipe {
    fn get_coord(&self) -> (usize, usize) {
        match self {
            Pipe::Vertical(x, y) => (*x, *y),
            Pipe::Horizontal(x, y) => (*x, *y),
            Pipe::BendNE(x, y) => (*x, *y),
            Pipe::BendNW(x, y) => (*x, *y),
            Pipe::BendSW(x, y) => (*x, *y),
            Pipe::BendSE(x, y) => (*x, *y),
            Pipe::Ground(x, y) => (*x, *y),
            Pipe::Start(x, y) => (*x, *y),
        }
    }
}

#[derive(Debug)]
struct PipeMap {
    map: Vec<Vec<Pipe>>,
}

impl PipeMap {
    fn get(&self, x: usize, y: usize) -> &Pipe {
        self.map
            .get(y)
            .expect("Could not get pipe")
            .get(x)
            .expect("Could not get pipe")
    }
}

fn parse_map(input: &str) -> ((usize, usize), PipeMap) {
    let mut map = Vec::new();
    let mut start_point = None;
    for (y, line) in input.lines().enumerate() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '|' => Pipe::Vertical(x, y),
                    '-' => Pipe::Horizontal(x, y),
                    'L' => Pipe::BendNE(x, y),
                    'J' => Pipe::BendNW(x, y),
                    '7' => Pipe::BendSW(x, y),
                    'F' => Pipe::BendSE(x, y),
                    'S' => {
                        start_point = Some((x, y));
                        Pipe::Start(x, y)
                    }
                    _ => Pipe::Ground(x, y),
                })
                .collect::<Vec<_>>(),
        );
    }
    (start_point.expect("Start not found"), PipeMap { map })
}

fn get_complete_pipe<'a>(start: &'a Pipe, map: &'a PipeMap) -> Vec<&'a Pipe> {
    let mut pipe = vec![start];
    let mut current_pipe = start;
    let (p1, _, _) = get_connected_pipes(current_pipe, map);

    let mut previous_pipe = current_pipe;
    current_pipe = p1;
    pipe.push(current_pipe);

    loop {
        let (p1, p2, _) = get_connected_pipes(current_pipe, map);
        if p1 == previous_pipe {
            previous_pipe = current_pipe;
            current_pipe = p2;
        } else {
            previous_pipe = current_pipe;
            current_pipe = p1;
        }
        if current_pipe == start {
            break;
        } else {
            pipe.push(current_pipe);
        }
    }
    pipe
}

fn get_connected_pipes<'a>(current_pipe: &'a Pipe, map: &'a PipeMap) -> (&'a Pipe, &'a Pipe, Pipe) {
    let pipe1: &Pipe;
    let pipe2: &Pipe;
    let mut pipe_type: Pipe = (*current_pipe).clone();
    match current_pipe {
        Pipe::Vertical(x, y) => {
            pipe1 = map.get(*x, *y - 1);
            pipe2 = map.get(*x, *y + 1);
        }
        Pipe::Horizontal(x, y) => {
            pipe1 = map.get(*x - 1, *y);
            pipe2 = map.get(*x + 1, *y);
        }
        Pipe::BendNE(x, y) => {
            pipe1 = map.get(*x + 1, *y);
            pipe2 = map.get(*x, *y - 1);
        }
        Pipe::BendNW(x, y) => {
            pipe1 = map.get(*x - 1, *y);
            pipe2 = map.get(*x, *y - 1);
        }
        Pipe::BendSW(x, y) => {
            pipe1 = map.get(*x - 1, *y);
            pipe2 = map.get(*x, *y + 1);
        }
        Pipe::BendSE(x, y) => {
            pipe1 = map.get(*x + 1, *y);
            pipe2 = map.get(*x, *y + 1);
        }
        Pipe::Ground(_, _) => {
            println!("Trying to get connected pipes from ground tile");
            pipe1 = current_pipe;
            pipe2 = current_pipe;
        }
        Pipe::Start(x, y) => {
            let mut pipes = Vec::new();
            let mut north = false;
            let mut south = false;
            let mut east = false;
            let mut west = false;

            let pipe = map.get(*x + 1, *y);
            if matches!(
                pipe,
                Pipe::Horizontal(_, _) | Pipe::BendNW(_, _) | Pipe::BendSW(_, _)
            ) {
                pipes.push(pipe);
                east = true;
            }

            if *x > 0 {
                let pipe = map.get(*x - 1, *y);
                if matches!(
                    pipe,
                    Pipe::Horizontal(_, _) | Pipe::BendNE(_, _) | Pipe::BendSE(_, _)
                ) {
                    pipes.push(pipe);
                    west = true;
                }
            }

            let pipe = map.get(*x, *y + 1);
            if matches!(
                pipe,
                Pipe::Vertical(_, _) | Pipe::BendSE(_, _) | Pipe::BendSW(_, _)
            ) {
                pipes.push(pipe);
                south = true;
            }
            if *y > 0 {
                let pipe = map.get(*x, *y - 1);
                if matches!(
                    pipe,
                    Pipe::Vertical(_, _) | Pipe::BendNW(_, _) | Pipe::BendNE(_, _)
                ) {
                    pipes.push(pipe);
                    north = true;
                }
            }

            assert_eq!(pipes.len(), 2);

            if north && south {
                pipe_type = Pipe::Vertical(0, 0);
            } else if east && west {
                pipe_type = Pipe::Horizontal(0, 0);
            } else if north && east {
                pipe_type = Pipe::BendNE(0, 0);
            } else if north && west {
                pipe_type = Pipe::BendNW(0, 0);
            } else if south && east {
                pipe_type = Pipe::BendSE(0, 0);
            } else if south && west {
                pipe_type = Pipe::BendSW(0, 0);
            }

            pipe1 = pipes[0];
            pipe2 = pipes[1];
        }
    }
    (pipe1, pipe2, pipe_type)
}

fn get_ground_tiles<'a>(map: &'a PipeMap, pipe: &'a [&'a Pipe]) -> Vec<&'a Pipe> {
    map.map
        .iter()
        .flat_map(|l| {
            l.iter()
                .filter(|p| !pipe.iter().any(|pp| pp == p))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn solve(input: &str) -> usize {
    let (start, map) = parse_map(input);
    let pipe = get_complete_pipe(map.get(start.0, start.1), &map);
    let ground_tiles = get_ground_tiles(&map, &pipe);

    let mut search_area = 0;
    for ground_tile in ground_tiles {
        let (start_x, start_y) = ground_tile.get_coord();
        let mut pipe_tiles = pipe
            .iter()
            .filter(|p| p.get_coord().0 == start_x && p.get_coord().1 < start_y)
            .collect::<Vec<_>>();

        pipe_tiles.sort_by(|a, b| a.get_coord().1.cmp(&b.get_coord().1));
        let mut start_pattern = None;
        let mut count = 0;
        for pipe_tile in pipe_tiles {
            match pipe_tile {
                Pipe::Horizontal(_, _) => {
                    count += 1;
                    start_pattern = None;
                }
                Pipe::BendSW(_, _) => start_pattern = Some((*pipe_tile).clone()),
                Pipe::BendSE(_, _) => start_pattern = Some((*pipe_tile).clone()),
                Pipe::BendNE(_, _) => {
                    if matches!(start_pattern, Some(Pipe::BendSW(_, _))) {
                        count += 1;
                        start_pattern = None;
                    }
                }
                Pipe::BendNW(_, _) => {
                    if matches!(start_pattern, Some(Pipe::BendSE(_, _))) {
                        count += 1;
                        start_pattern = None;
                    }
                }
                Pipe::Start(_, _) => {
                    let (_, _, pt) = get_connected_pipes(pipe_tile, &map);
                    match pt {
                        Pipe::Horizontal(_, _) => {
                            count += 1;
                            start_pattern = None;
                        }
                        Pipe::BendNE(_, _) => {
                            if matches!(start_pattern, Some(Pipe::BendSW(_, _))) {
                                count += 1;
                                start_pattern = None;
                            }
                        }
                        Pipe::BendNW(_, _) => {
                            if matches!(start_pattern, Some(Pipe::BendSE(_, _))) {
                                count += 1;
                                start_pattern = None;
                            }
                        }
                        Pipe::BendSW(_, _) => start_pattern = Some(pt.clone()),
                        Pipe::BendSE(_, _) => start_pattern = Some(pt.clone()),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        if count % 2 == 1 {
            search_area += 1;
        }
    }

    search_area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let res = solve(input);
        assert_eq!(res, 4);
    }

    #[test]
    fn it_works_2() {
        let input = include_str!("./inputs/test-day10");
        let res = solve(input);
        assert_eq!(res, 10);
    }
}
