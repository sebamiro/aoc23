use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn race(time: u64, distance: u64) -> u64 {
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

    let mut time: u64 = 0;
    let mut distance: u64 = 0;
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    if time == 0 {
                        let (_, times) = s.split_once(':').unwrap();
                        let times: String = times.chars().filter(|c| !c.is_whitespace()).collect();
                        time = times.parse::<u64>().unwrap();
                    } else {
                        let (_, distances) = s.split_once(':').unwrap();
                        let distances: String = distances.chars().filter(|c| !c.is_whitespace()).collect();
                        distance = distances.parse::<u64>().unwrap();
                    }
                }
            }
        }
    }
    println!("{} {}", time, distance);
    let n = race(time, distance);
    println!("Result: {}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
