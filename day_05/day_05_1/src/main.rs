use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn map_seeds(seeds: &Vec<u64>, temp: &mut Vec<u64>, s: String) -> Vec<u64> {
    let split: Vec<&str> = s.splitn(3, ' ').collect();
    let (dest_s, src_s, len) = (split[0].parse::<u64>().unwrap(),
                                split[1].parse::<u64>().unwrap(),
                                split[2].parse::<u64>().unwrap());
    seeds.iter().map(|x| {
        if !temp.contains(x) && *x >= src_s && *x < src_s + len {
            temp.push(*x + dest_s - src_s);
            *x + dest_s - src_s
        } else {
            *x
        }
    }).collect()
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    let mut seeds: Vec<u64> = Vec::new();
    let mut temp: Vec<u64> = Vec::new();
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    if seeds.len() == 0 {
                        let (_, s) = s.split_once(':').unwrap();
                        seeds = s.trim().split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                    } else if s.starts_with(|c: char| c.is_digit(10)) {
                        seeds = map_seeds(&seeds, &mut temp, s);
                    } else {
                        temp.clear();
                    }
                }
            }
        }
    }
    let n = seeds.iter().min().unwrap();
    println!("Result: {}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
