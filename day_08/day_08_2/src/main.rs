use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug)]
struct Node {
    left: String,
    right: String
}

#[derive(Debug)]
struct Ghost<'a> {
    current: &'a str,
    found: usize
}

fn new_node(s: String) -> (String, Node) {
    let (key, value) = s.split_once('=').unwrap();
    let value = value.trim().trim_matches(|c| "()".contains(c));
    let (left, right) = value.split_once(',').unwrap();
    let left = left.trim().to_string();
    let right = right.trim().to_string();
    (key.trim().to_string(), Node{ left, right })
}

fn check_ghosts(ghosts: &Vec<Ghost>) -> bool {
    let z = ghosts.iter().filter(|g| g.current.chars().last().unwrap() == 'Z').count();
    z == ghosts.len()
}

fn gdc(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    let mut net: HashMap<String, Node> = HashMap::new();
    let mut dir: String = String::new();
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    if dir.len() == 0 {
                        dir = s;
                        continue;
                    }
                    let (key, node) = new_node(s);
                    eprintln!("[{key}]: {:?}", node);
                    net.insert(key, node);
                }
            }
        }
    }
    let mut ghosts: Vec<Ghost> = Vec::new();
    net.iter().for_each(|(k, _)| if k.chars().last().unwrap() == 'A' { ghosts.push(Ghost{ current: k, found: 0 })});
    eprintln!("{:?}", ghosts);
    let mut n = 0;
    while !check_ghosts(&ghosts) {
        for c in dir.chars() {
            n += 1;
            for g in ghosts.iter_mut() {
                if g.found != 0 {
                    continue;
                }
                if c == 'R' {
                    g.current = net[g.current].right.as_str();
                } else {
                    g.current = net[g.current].left.as_str();
                }
                if g.current.chars().last().unwrap() == 'Z' {
                    g.found = n;
                }
            }
        }
    }
    eprintln!("{:?}", ghosts);
    let lcm = |a: usize, b: usize| a * b / gdc(a, b);
    n = 1;
    for g in ghosts.iter() {
        n = lcm(g.found, n);
    }
    println!("Result: {}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
