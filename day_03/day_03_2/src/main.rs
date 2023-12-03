use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

struct Vector2 {
    x: usize,
    y: usize
}

struct Number {
    i_x: usize,
    e_x: usize,
    y: usize,
    n: u32
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    let mut symbol_vec: Vec<Vector2> = Vec::new();
    let mut num_vec: Vec<Number> = Vec::new();
    if let Ok(lines) = read_lines(args[1].clone()) {
        let lines: Vec<String> = lines.flatten().collect();
        for (y, line) in lines.iter().enumerate() {
            let mut in_n = false;
            for (x, c) in line.chars().enumerate() {
                if c.is_digit(10) && !in_n {
                    num_vec.push(Number{i_x: x, e_x: x, y, n: c.to_digit(10).unwrap()});
                    in_n = true;
                }
                else if c.is_digit(10) && in_n {
                    if let Some(last) = num_vec.last_mut() {
                        last.n = last.n * 10 + c.to_digit(10).unwrap();
                        last.e_x = x;
                    }
                }
                else {
                    in_n = false;
                }
                if c == '*' {
                    symbol_vec.push(Vector2{x, y});
                }
            }
        }
    }
    let mut n = 0;
    for symbol in symbol_vec {
        let mut z = 0;
        for num in num_vec.iter() {
            if num.y >= symbol.y - 1 && num.y <= symbol.y + 1 {
                if symbol.x - 1 >= num.i_x && symbol.x - 1 <= num.e_x ||
                    symbol.x + 1 >= num.i_x && symbol.x + 1 <= num.e_x ||
                    symbol.x == num.i_x {
                        if z != 0 {
                            n += z * num.n;
                            z = 0;
                        } else {
                            z = num.n;
                        }
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
