use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Clone)]
struct Map {
    dest: u64,
    src: u64,
    len: u64
}

fn crate_map(s: String) -> Map {
    let split: Vec<&str> = s.splitn(3, ' ').collect();
    let (dest, src, len) = (split[0].parse::<u64>().unwrap(),
                                split[1].parse::<u64>().unwrap(),
                                split[2].parse::<u64>().unwrap());
    Map{dest, src, len}
}

fn map_seed(seed: u64, map: &Vec<Map>) -> u64 {
    for m in map {
        if seed >= m.dest && seed < m.dest + m.len {
            return m.src + (seed - m.dest);
        }
    }
    seed
}

fn find_res(seeds: Vec<u64>, maps: Vec<Vec<Map>>) -> u64 {
    let mut i: u64 = 217504920;
    loop {
        let mut x = i;

        println!("{}", i);
        for m in maps.iter().rev() {
            x = map_seed(x, &m);
        }
        for s in (0..seeds.len()).step_by(2) {
            if x >= seeds[s] && x < seeds[s] + seeds[s + 1] {
                return i;
            }
        }
        i += 1;
    }
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    let mut seeds: Vec<u64> = Vec::new();
    let mut temp_maps: Vec<Map> = Vec::new();
    let mut maps: Vec<Vec<Map>> = Vec::new();
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    if seeds.len() == 0 {
                        let (_, s) = s.split_once(':').unwrap();
                        seeds = s.trim().split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                    } else if s.starts_with(|c: char| c.is_digit(10)) {
                        temp_maps.push(crate_map(s));
                    } else {
                        if temp_maps.len() > 0 {
                            maps.push(temp_maps.clone());
                        }
                        temp_maps.clear();
                    }
                }
            }
        }
    }
    let n = find_res(seeds, maps);
    println!("Result: {}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
