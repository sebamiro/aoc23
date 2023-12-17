use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug, Clone, Copy)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP
}

#[derive(Debug)]
struct Player {
    x: i32,
    y: i32,
    straight_moves: i32,
    dir: Direction,
    heat: i32
}

#[derive(Debug)]
enum State {
    WIN(Player),

}

impl Player {
    fn new(x: i32, y: i32, straight_moves: i32, dir: Direction, heat: i32) -> Self {
        Self { x, y, straight_moves, dir, heat }
    }

    fn is_valid(&self, map: &Vec<Vec<char>>) -> bool {
        match self.dir {
            Direction::RIGHT => (self.x as usize) + 1 < map[0].len(),
            Direction::DOWN => (self.y as usize) + 1 < map.len(),
            Direction::LEFT => self.x - 1 >= 0,
            Direction::UP => self.y - 1 >= 0
        }
    }

    fn straight(&self, map: &Vec<Vec<char>>) -> Option<Self> {
        if self.straight_moves == 3 || !self.is_valid(map) {
            return None;
        }
        let heat = self.heat + map[self.x as usize][self.y as usize].to_digit(10).unwrap() as i32;
        match self.dir {
            Direction::RIGHT => Some(Self::new(self.x + 1, self.y, self.straight_moves + 1, self.dir, heat)),
            Direction::DOWN => Some(Self::new(self.x, self.y + 1, self.straight_moves + 1, self.dir, heat)),
            Direction::LEFT => Some(Self::new(self.x - 1, self.y, self.straight_moves + 1, self.dir, heat)),
            Direction::UP => Some(Self::new(self.x, self.y - 1, self.straight_moves + 1, self.dir, heat)),
        }
    }
}

fn move_p(player: Player, map: &Vec<Vec<char>>) -> Player {
    let x = player.x as usize;
    let y = player.y as usize;
    if x == map[0].len() - 1 && y == map.len() {
        return player;
    }
    return player;
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(args[1].clone()) {
        let n = 0;
        let map: Vec<Vec<char>> = lines.flatten().filter(|s| !s.is_empty()).map(|s| s.chars().collect()).collect();
        let mut player = Player::new(0, 0, 0, Direction::RIGHT, 0);
        while let Some(p) = player.straight(&map) {
            eprintln!("{p:?}");
            player = p;
        }
        for m in map {
            eprintln!("{m:?}");
        }
        println!("Result: {}", n);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
