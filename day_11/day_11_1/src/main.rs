use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize
}

fn expand_rows(map: Vec<String>) -> Vec<String> {
    let mut new: Vec<String> = Vec::new();
    for v in map.iter() {
        if !v.contains('#') {
            new.push(v.clone());
        }
        new.push(v.clone());
    }
    new
}

fn expand_cols(mut map: Vec<String>) -> Vec<String> {
    let len = map[0].len();
    for i in (0..len).rev() {
        if map.iter().filter(|s| s.chars().nth(i).unwrap() != '.').count() == 0 {
            map.iter_mut().for_each(|s| s.insert(i, '.'));
        }
    }
    map
}

fn distance(g1: &Galaxy, g2: &Galaxy) -> usize {
    g1.x.abs_diff(g2.x) + g1.y.abs_diff(g2.y)
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(args[1].clone()) {
        let map: Vec<String> = lines.flat_map(|x| x.ok()).filter(|x| x.len() > 0).collect();
        eprintln!("{:#?}", map);
        let map = expand_rows(map);
        let map = expand_cols(map);
        eprintln!("{:#?}", map);
        let mut galaxies: Vec<Galaxy> = Vec::new();
        for (y, m) in map.iter().enumerate() {
            for (x, c) in m.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Galaxy { x, y });
                }
            }
        }
        eprintln!("{:?}", galaxies);
        let mut n = 0;
        while let Some(g) = galaxies.pop() {
            for galaxy in galaxies.iter() {
                n += distance(&g, &galaxy);
            }
        }
        println!("Result: {}", n);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
