use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn hash(n: u32) -> u32 {
    (n * 17) % 256
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
                    let steps = s.split(',');
                    for s in steps {
                        eprintln!("{s}");
                        let mut x = 0;
                        for c in s.bytes() {
                            x = hash(x + c as u32);
                            eprintln!("{x}");
                        }
                        n += x;
                    }
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
