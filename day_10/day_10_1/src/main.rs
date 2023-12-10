use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug)]
struct Pipe {
    c: char,
    x: usize,
    y: usize,
    hottness: usize
}

fn new_pipe_map(x: usize, y: usize, map: &mut Vec<String>, pipes: &mut Vec<Pipe>) {
    if let Some(c) = map[y].chars().nth(x) {
        if pipes.iter().filter(|p| p.x == x && p.y == y).count() > 0 {
            return ;
        }
        pipes.push(Pipe{ c, x, y, hottness: pipes.len() });
        if y != 0 {
            if let Some(n) = map[y - 1].chars().nth(x) {
                if "S|F7".contains(n) && "S|LJ".contains(c) {
                    new_pipe_map(x, y - 1, map, pipes);
                }
            }
        }
        if y + 1 < map.len() {
            if let Some(n) = map[y + 1].chars().nth(x) {
                if "S|LJ".contains(n) && "S|F7".contains(c) {
                    new_pipe_map(x, y + 1, map, pipes);
                }
            }
        }
        if x != 0 {
            if let Some(n) = map[y].chars().nth(x - 1) {
                if "S-LF".contains(n) && "S-J7".contains(c) {
                    new_pipe_map(x - 1, y, map, pipes);
                }
            }
        }
        if let Some(n) = map[y].chars().nth(x + 1) {
            if "S-J7".contains(n) && "S-LF".contains(c) {
                new_pipe_map(x + 1, y, map, pipes);
            }
        }
    }
}

fn hotmap(i: usize, pipe: &mut Pipe) {
    if pipe.hottness > i {
        pipe.hottness = i;
    }
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    let mut line_map: Vec<String> = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    if let Some(x) = s.find('S') {
                        start_x = x;
                        start_y = line_map.len();
                    }
                    line_map.push(s);
                }
            }
        }
    }
    let mut pipe_map: Vec<Pipe> = Vec::new();
    new_pipe_map(start_x, start_y, &mut line_map, &mut pipe_map);
    let mut n = 0;
    for (i, v) in pipe_map.iter_mut().rev().enumerate() {
        hotmap(i + 1, v);
        if v.hottness > n {
            n = v.hottness;
        }
    }
    eprintln!("{:#?}", pipe_map);
    println!("Result: {}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
