use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn get_number(s: &str) -> u32 {
    let mut numbers = s.chars().filter(|c| { c.is_digit(10) });
    let mut res = 0;
    if let Some(first) = numbers.next() {
        res = first.to_digit(10).unwrap();
    };
    if let Some(last) = numbers.last() {
        res = res * 10 + last.to_digit(10).unwrap();
    } else {
        res = res * 10 + res;
    };
    eprintln!("s: {}: {}", s, res);
    res
}
fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }

    let mut n = 0;
    let args: Vec<String> = env::args().collect();
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                n += get_number(s.as_str());
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
