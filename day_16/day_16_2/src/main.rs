use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP
}

#[derive(Debug)]
struct Ray {
    x: i32,
    y: i32,
    dir: Direction
}

fn move_ray(ray: Ray, map: &Vec<Vec<char>>, visited: &mut Vec<Vec<char>>) {
    if ray.x < 0 || ray.x as usize >= map[0].len()
        || ray.y < 0 || ray.y as usize >= map.len() {
        return ;
    }
    if visited[ray.y as usize][ray.x as usize]  == '#' &&
        "|-".contains(map[ray.y as usize][ray.x as usize]) {
        return ;
    }
    visited[ray.y as usize][ray.x as usize] = '#';
    match map[ray.y as usize][ray.x as usize] {
        '|' => match ray.dir {
            Direction::RIGHT |
            Direction::LEFT => {
                move_ray(Ray{ x: ray.x, y: ray.y - 1, dir: Direction::UP}, map, visited);
                move_ray(Ray{ x: ray.x, y: ray.y + 1, dir: Direction::DOWN}, map, visited);
                return ;
            },
            Direction::DOWN => move_ray(Ray{ x: ray.x, y: ray.y + 1, dir: ray.dir}, map, visited),
            Direction::UP => move_ray(Ray{ x: ray.x, y: ray.y - 1, dir: ray.dir}, map, visited),
        },
        '-' => match ray.dir {
            Direction::UP |
            Direction::DOWN => {
                move_ray(Ray{ x: ray.x + 1, y: ray.y, dir: Direction::RIGHT}, map, visited);
                move_ray(Ray{ x: ray.x - 1, y: ray.y, dir: Direction::LEFT}, map, visited);
                return ;
            },
            Direction::RIGHT => move_ray(Ray{ x: ray.x + 1, y: ray.y, dir: ray.dir}, map, visited),
            Direction::LEFT => move_ray(Ray{ x: ray.x - 1, y: ray.y, dir: ray.dir}, map, visited),
        },
        '/' => match ray.dir {
            Direction::RIGHT => move_ray(Ray{ x: ray.x, y: ray.y - 1, dir: Direction::UP}, map, visited),
            Direction::LEFT => move_ray(Ray{ x: ray.x, y: ray.y + 1, dir: Direction::DOWN}, map, visited),
            Direction::DOWN => move_ray(Ray{ x: ray.x - 1, y: ray.y, dir: Direction::LEFT}, map, visited),
            Direction::UP => move_ray(Ray{ x: ray.x + 1, y: ray.y, dir: Direction::RIGHT}, map, visited),
        },
        '\\' => match ray.dir {
            Direction::RIGHT => move_ray(Ray{ x: ray.x, y: ray.y + 1, dir: Direction::DOWN}, map, visited),
            Direction::LEFT => move_ray(Ray{ x: ray.x, y: ray.y - 1, dir: Direction::UP}, map, visited),
            Direction::DOWN => move_ray(Ray{ x: ray.x + 1, y: ray.y, dir: Direction::RIGHT}, map, visited),
            Direction::UP => move_ray(Ray{ x: ray.x - 1, y: ray.y, dir: Direction::LEFT}, map, visited),
        },
        _ => match ray.dir {
            Direction::RIGHT => move_ray(Ray{ x: ray.x + 1, y: ray.y, dir: ray.dir}, map, visited),
            Direction::LEFT => move_ray(Ray{ x: ray.x - 1, y: ray.y, dir: ray.dir}, map, visited),
            Direction::DOWN => move_ray(Ray{ x: ray.x, y: ray.y + 1, dir: ray.dir}, map, visited),
            Direction::UP => move_ray(Ray{ x: ray.x, y: ray.y - 1, dir: ray.dir}, map, visited),
        }
    }
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(args[1].clone()) {
        let mut n = 0;
        let map: Vec<Vec<char>> = lines.flatten().filter(|s| !s.is_empty()).map(|s| s.chars().collect()).collect();
        for r in 0..2 {
            for i in 0..map.len() {
                let mut t = 0;
                let mut visited = map.clone();
                if r == 0 {
                    move_ray(Ray{ x: 0 , y: i as i32, dir: Direction::RIGHT}, &map, &mut visited);
                } else {
                    move_ray(Ray{ x: map[0].len() as i32 - 1 , y: i as i32, dir: Direction::LEFT}, &map, &mut visited);
                }
                for x in visited {
                    t += x.iter().filter(|c| **c == '#').count();
                }
                if n < t {
                    n = t;
                }
            }
        }
        for r in 0..2 {
            for i in 0..map[0].len() {
                let mut t = 0;
                let mut visited = map.clone();
                if r == 0 {
                    move_ray(Ray{ x: i as i32, y: 0, dir: Direction::DOWN}, &map, &mut visited);
                } else {
                    move_ray(Ray{ x: i as i32, y: map.len() as i32 - 1, dir: Direction::UP}, &map, &mut visited);
                }
                for x in visited {
                    t += x.iter().filter(|c| **c == '#').count();
                }
                if n < t {
                    n = t;
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
