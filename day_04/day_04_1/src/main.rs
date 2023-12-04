use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn manage_game(s: String) -> u32 {
    let (_, s) = s.split_once(':').unwrap();
    let (winning, numbers) = s.split_once('|').unwrap();
    let winning: Vec<u32> = winning.trim().split_whitespace()
        .map(|n| {n.parse::<u32>().unwrap()}).collect();
    let numbers: Vec<u32> = numbers.trim().split_whitespace()
        .map(|n| {n.parse::<u32>().unwrap()}).collect();
    let mut n = 0;
    winning.iter().for_each(|x|
                              if numbers.contains(x) {
                                  if n == 0 { n = 1 } else { n *= 2}
                              });
    n
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    let mut n = 0;
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    n += manage_game(s);
                }
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
