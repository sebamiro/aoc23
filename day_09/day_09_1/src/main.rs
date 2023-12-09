use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn rec_add_next(v: &mut Vec<i64>) -> &Vec<i64> {
    if v.iter().filter(|x| **x != 0).count() == 0 {
        return v;
    }
    let mut n_v: Vec<i64> = Vec::new();

    for i in 0..v.len() - 1 {
        n_v.push(v[i + 1] - v[i]);
    }
    let x = rec_add_next(&mut n_v);
    v.push(v.iter().last().unwrap() + x.iter().last().unwrap());
    v
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
                    let mut v: Vec<i64> = s.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
                    let v = rec_add_next(&mut v);
                    eprintln!("{:?}", v);
                    n += v.last().unwrap();
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
