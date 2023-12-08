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

fn new_node(s: String) -> (String, Node) {
    let (key, value) = s.split_once('=').unwrap();
    let value = value.trim().trim_matches(|c| "()".contains(c));
    let (left, right) = value.split_once(',').unwrap();
    let left = left.trim().to_string();
    let right = right.trim().to_string();
    (key.trim().to_string(), Node{ left, right })
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
    let mut n = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        for c in dir.chars() {
            n += 1;
            if c == 'R' {
                current = net[current].right.as_str();
            } else {
                current = net[current].left.as_str();
            }
            eprintln!("{current} {c}");
            if current == "ZZZ" {
                break;
            }
        }
    }
    println!("Result: {}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
