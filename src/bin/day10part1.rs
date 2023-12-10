fn main() {
    let input = include_str!("./inputs/day10");
    let res = solve(input);
    dbg!(res);
}

#[derive(Debug, PartialEq)]
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

fn get_complete_pipe_length(start: &Pipe, map: &PipeMap) -> usize {
    let mut pipe_size = 0;
    let mut current_pipe = start;
    let (p1, _) = get_connected_pipes(current_pipe, map);

    let mut previous_pipe = current_pipe;
    current_pipe = p1;

    loop {
        let (p1, p2) = get_connected_pipes(current_pipe, map);
        if p1 == previous_pipe {
            previous_pipe = current_pipe;
            current_pipe = p2;
        } else {
            previous_pipe = current_pipe;
            current_pipe = p1;
        }
        pipe_size += 1;
        if current_pipe == start {
            break;
        }
    }
    dbg!(pipe_size + 1)
}

fn get_connected_pipes<'a>(current_pipe: &'a Pipe, map: &'a PipeMap) -> (&'a Pipe, &'a Pipe) {
    let pipe1: &Pipe;
    let pipe2: &Pipe;
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
            let pipe = map.get(*x + 1, *y);
            match pipe {
                Pipe::Horizontal(_, _) => pipes.push(pipe),
                Pipe::BendNW(_, _) => pipes.push(pipe),
                Pipe::BendSW(_, _) => pipes.push(pipe),
                _ => {}
            }
            if *x > 0 {
                let pipe = map.get(*x - 1, *y);
                match pipe {
                    Pipe::Horizontal(_, _) => pipes.push(pipe),
                    Pipe::BendNE(_, _) => pipes.push(pipe),
                    Pipe::BendSE(_, _) => pipes.push(pipe),
                    _ => {}
                }
            }

            let pipe = map.get(*x, *y + 1);
            match pipe {
                Pipe::Vertical(_, _) => pipes.push(pipe),
                Pipe::BendSE(_, _) => pipes.push(pipe),
                Pipe::BendSW(_, _) => pipes.push(pipe),
                _ => {}
            }
            if *y > 0 {
                let pipe = map.get(*x, *y - 1);
                match pipe {
                    Pipe::Vertical(_, _) => pipes.push(pipe),
                    Pipe::BendNW(_, _) => pipes.push(pipe),
                    Pipe::BendNE(_, _) => pipes.push(pipe),
                    _ => {}
                }
            }

            assert_eq!(pipes.len(), 2);
            pipe1 = pipes[0];
            pipe2 = pipes[1];
        }
    }
    (pipe1, pipe2)
}

fn solve(input: &str) -> usize {
    let (start, map) = parse_map(input);
    let pipe_length = get_complete_pipe_length(map.get(start.0, start.1), &map);
    dbg!(pipe_length);
    pipe_length / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let res = solve(input);
        assert_eq!(res, 8);
    }
}
