use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug, Clone, Eq)]
struct Vector2 {
    x: i32,
    y: i32
}

impl Ord for Vector2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.x.cmp(&other.x),
            Ordering::Greater => Ordering::Greater
        }
    }
}

impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn

fn move_circle(c: Vector2, circles: &Vec<Vector2>, square: &Vec<Vector2>, dir: &Vector2, limit: &Vector2) -> Vector2 {
    let mut res = Vector2{ x: c.x, y: c.y};
    loop {
        if circles.iter().filter(|x| x.x == res.x + dir.x && x.y == res.y + dir.y).count() > 0 ||
            square.iter().filter(|x| x.x == res.x + dir.x && x.y == res.y + dir.y).count() > 0 ||
            res.x + dir.x < 0 || res.x + dir.x >= limit.x || res.y + dir.y < 0 || res.y + dir.y >= limit.y {
            break;
        }
        res.x += dir.x;
        res.y += dir.y;
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
        let mut rows = 0;
        let mut circles: Vec<Vector2> = Vec::new();
        let mut squares: Vec<Vector2> = Vec::new();
        let mut limit = Vector2{ x: 0, y: 0 };
        let dirs: Vec<Vector2> = vec!(Vector2{ x: 0, y: -1}, Vector2{ x: -1, y: 0}, Vector2{ x: 0, y: 1}, Vector2{ x: 1, y: 0});
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    for (i, c) in s.chars().enumerate() {
                        match c {
                            'O' => circles.push(Vector2{ x: i as i32, y: rows}),
                            '#' => squares.push(Vector2{ x: i as i32, y: rows}),
                            _ => ()
                        }
                    }
                    rows += 1;
                    limit.x = s.len() as i32;
                }
            }
        }
        limit.y = rows as i32;
        let mut i = 0;
        loop {
            let mut n = 0;
            for (i, dir) in dirs.iter().enumerate() {
                circles.sort();
                if i <= 1 {
                    circles.reverse();
                }
                let mut len = circles.len();
                while let Some(x) = circles.pop() {
                    circles.insert(0, move_circle(x, &circles, &squares, dir, &limit));
                    len -= 1;
                    if len == 0 {
                        break;
                    }
                }
                //eprintln!("{:?}\n", circles);
            }
            circles.iter().for_each(|x| n += rows - x.y);
            println!("[{i}] Result: {}", n);
            i += 1;
            if i == 1000000000 {
                break;
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
