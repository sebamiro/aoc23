use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize
}

fn expand_rows(map: &Vec<String>) -> Vec<usize> {
    let mut new: Vec<usize> = Vec::new();
    for (i, v) in map.iter().enumerate() {
        if !v.contains('#') {
            new.push(i);
        }
    }
    new
}

fn expand_cols(map: &Vec<String>) -> Vec<usize> {
    let len = map[0].len();
    let mut new: Vec<usize> = Vec::new();
    for i in 0..len {
        if map.iter().filter(|s| s.chars().nth(i).unwrap() != '.').count() == 0 {
            new.push(i);
        }
    }
    new
}

fn expand(g: &mut Galaxy, rows: &Vec<usize>, cols: &Vec<usize>) {
    let x = g.x;
    let y = g.y;
    cols.iter().for_each(|n| {
        if x > *n { g.x += 999999 };
    });
    rows.iter().for_each(|n| {
        if y > *n { g.y += 999999 };
    });
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
        let rows = expand_rows(&map);
        let cols = expand_cols(&map);
        eprintln!("{:#?}", rows);
        eprintln!("{:#?}", cols);
        let mut galaxies: Vec<Galaxy> = Vec::new();
        for (y, m) in map.iter().enumerate() {
            for (x, c) in m.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Galaxy { x, y });
                }
            }
        }
        eprintln!("{:?}", galaxies);
        galaxies.iter_mut().for_each(|g| expand(g, &rows, &cols));
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
