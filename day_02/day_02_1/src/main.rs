use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn check_game(s: String) -> i32 {
    if let Some((game, cubes)) = s.split_once(':') {
        println!("[{}] - [{}]", game, cubes);
        let (_, id) = game.split_once(' ').unwrap();
        println!("ID={}", id);
        let sets = cubes.split(';');
        for set in sets {
            println!("SET={}", set);
            let colors = set.split(',').map(str::trim);
            for color in colors {
                println!("COLOR={}", color);
                let (n, name) = color.split_once(' ').unwrap();
                if name == "red" && n.parse::<i32>().unwrap() > 12 {
                    return 0;
                }
                if name == "green" && n.parse::<i32>().unwrap() > 13 {
                    return 0;
                }
                if name == "blue" && n.parse::<i32>().unwrap() > 14 {
                    return 0;
                }
            }
        }
        return id.parse::<i32>().unwrap();
    }
    0
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
