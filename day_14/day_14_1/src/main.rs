use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug, Clone)]
struct Vector2 {
    x: usize,
    y: usize
}

fn move_circle(c: Vector2, circles: &Vec<Vector2>, square: &Vec<Vector2>) -> Vector2 {
    let mut res = Vector2{ x: c.x, y: c.y};

    while res.y > 0 {
        if circles.iter().filter(|x| x.x == res.x && x.y == res.y - 1).count() > 0 ||
            square.iter().filter(|x| x.x == res.x && x.y == res.y - 1).count() > 0 {
            break;
        }
        res.y -= 1;
    }
    res
}
fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(args[1].clone()) {
        let mut n = 0;
        let mut rows = 0;
        let mut circles: Vec<Vector2> = Vec::new();
        let mut squares: Vec<Vector2> = Vec::new();
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    for (i, c) in s.chars().enumerate() {
                        match c {
                            'O' => circles.push(Vector2{ x: i, y: rows}),
                            '#' => squares.push(Vector2{ x: i, y: rows}),
                            _ => ()
                        }
                    }
                    rows += 1;
                }
            }
        }
        eprintln!("{:?}", circles);
        circles.reverse();
        let mut len = circles.len();
        while let Some(x) = circles.pop() {
            circles.insert(0, move_circle(x, &circles, &squares));
            len -= 1;
            if len == 0 {
                break;
            }
        }
        circles.reverse();
        eprintln!("{:?}", circles);
        circles.iter().for_each(|x| n += rows - x.y);
        println!("Result: {}", n);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
