use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

fn is_smudge_horizontal(s1: &String, s2: &String) -> bool {
    let mut n = 0;
    for (i, c) in s1.chars().enumerate() {
        if c != s2.chars().nth(i).unwrap() {
            n += 1;
        }
        if n > 1 {
            return false;
        }
    }
    true
}

fn is_vertical_mirror(pattern: &Vec<String>) -> i32 {
    let mut i = 0;
    let mut j = 1;

    while j < pattern[0].len() {
        let mut x = 0;
        let mut smudge1 = false;
        while x < pattern.len() &&
            (pattern[x].chars().nth(i).unwrap() == pattern[x].chars().nth(j).unwrap()
             || !smudge1 ) {
            if pattern[x].chars().nth(i).unwrap() != pattern[x].chars().nth(j).unwrap() {
                smudge1 = true;
            }
            x += 1;
        }
        if x == pattern.len() {
            let mut x = i;
            let mut y = j;
            while y < pattern[0].len() {
                let mut z = 0;
                while z < pattern.len() &&
                    (pattern[z].chars().nth(x).unwrap() == pattern[z].chars().nth(y).unwrap() ||
                     !smudge1) {
                    if pattern[z].chars().nth(x).unwrap() != pattern[z].chars().nth(y).unwrap() {
                        smudge1 = true;
                    }
                    z += 1;
                }
                if z != pattern.len() {
                    break;
                }
                if x == 0 {
                    y = pattern[0].len();
                    break;
                }
                x -= 1;
                y += 1;
            }
            if y == pattern[0].len() {
                return (i + 1) as i32;
            }
        }
        j += 1;
        i += 1;
    }
    return -1;
}

fn is_horizontal_mirror(pattern: &Vec<String>) -> i32 {
    let mut i = 0;
    let mut j = 1;

    while j < pattern.len() {
        let mut smudge1 = is_smudge_horizontal(&pattern[i], &pattern[j]);
        if pattern[i] == pattern[j] || smudge1 {
            let mut x = i;
            let mut y = j;
            while y < pattern.len() {
                if pattern[x] != pattern[y] && !smudge1 && !is_smudge_horizontal(&pattern[x], &pattern[y]) {
                    break ;
                }
                if pattern[x] != pattern[y] {
                    smudge1 = false;
                }
                if x == 0 {
                    y = pattern.len();
                    break;
                }
                x -= 1;
                y += 1;
            }
            if y == pattern.len() {
                return (i + 1) as i32;
            }
        }
        i += 1;
        j += 1;
    }
    return -1;
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(args[1].clone()) {
        let mut pattern: Vec<String> = Vec::new();
        let mut n = 0;
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    pattern.push(s);
                } else {
                    eprintln!("{:#?}", pattern);
                    let h = is_horizontal_mirror(&pattern);
                    let v = is_vertical_mirror(&pattern);
                    if h == -1 { n += v } else { n += h * 100};
                    pattern.clear();
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
