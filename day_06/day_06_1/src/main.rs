use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn race(time: u32, distance: u32) -> u32 {
    let mut i = 0;
    let mut ret = 0;

    while i < time {
        if (time - i) * i > distance {
            ret += 1;
        }
        i += 1;
    }
    ret
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    let mut time: Vec<u32> = Vec::new();
    let mut distance: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    if time.len() == 0 {
                        let (_, times) = s.split_once(':').unwrap();
                        time = times.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
                    } else {
                        let (_, distances) = s.split_once(':').unwrap();
                        distance = distances.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
                    }
                }
            }
        }
    }
    let mut n = race(time[0], distance[0]);
    for i in 1..time.len() {
        n *= race(time[i], distance[i]);
    }
    println!("Result: {}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
