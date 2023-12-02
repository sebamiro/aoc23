use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn check_game(s: String) -> i32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    if let Some((_, cubes)) = s.split_once(':') {
        let sets = cubes.split(';');
        for set in sets {
            println!("SET={}", set);
            let colors = set.split(',').map(str::trim);
            for color in colors {
                println!("COLOR={}", color);
                let (n, name) = color.split_once(' ').unwrap();
                let n = n.parse::<i32>().unwrap();
                if name == "red" && n > red {
                    red = n;
                }
                if name == "green" && n > green {
                    green = n;
                }
                if name == "blue" && n > blue {
                    blue = n;
                }
            }
        }
    }
    red * green * blue
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
                n += check_game(s);
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
