use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn is_valid(s: &str, n: &Vec<usize>) -> bool {
    let damaged: Vec<&str> = s.split('.').filter(|x| !x.is_empty()).collect();
    if damaged.len() != n.len() {
        return false;
    }
    for i in 0..damaged.len() {
        if damaged[i].len() != n[i] {
            return false;
        }
    }
    true
}

fn rec_fn(s: &str, n: &Vec<usize>) -> usize {
    if !s.contains('?') {
        return if is_valid(s, n) { 1 } else { 0 };
    }
    if let Some(i) = s.find('?') {
        let mut r = 0;
        let mut s1 = s.to_string();
        s1.replace_range(i..i + 1, ".");
        r += rec_fn(s1.as_str(), n);
        s1.replace_range(i..i + 1, "#");
        r += rec_fn(s1.as_str(), n);
        return r;
    }
    0
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(args[1].clone()) {
        let mut n = 0;
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    eprintln!("{s}");
                    let (springs, damaged) = s.split_once(' ').unwrap();
                    let damaged: Vec<usize> = damaged.split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect();
                    let amount = rec_fn(springs, &damaged);
                    eprintln!("{amount}");
                    n += amount;
                }
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
