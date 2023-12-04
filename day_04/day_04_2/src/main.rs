use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Clone)]
struct Game {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
    instances: u32,
}

fn create_game(s: String) -> Game {
    let (game, s) = s.split_once(':').unwrap();
    let (_, id) = game.split_once(' ').unwrap();
    let (winning, numbers) = s.split_once('|').unwrap();
    let winning: Vec<u32> = winning.trim().split_whitespace()
        .map(|n| {n.parse::<u32>().unwrap()}).collect();
    let numbers: Vec<u32> = numbers.trim().split_whitespace()
        .map(|n| {n.parse::<u32>().unwrap()}).collect();
    Game{id: id.trim().parse::<u32>().unwrap(), winning, numbers, instances: 1}
}

fn manage_game(g: Game, games: &mut Vec<Game>) {
    let mut n = 0;
    g.winning.iter().for_each(|x|
                              if g.numbers.contains(x) {
                                  games[(g.id + n) as usize].instances += 1 * g.instances;
                                  n += 1;
                              });
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    let mut games: Vec<Game> = Vec::new();
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    games.push(create_game(s));
                }
            }
        }
    }
    for i in 0..games.len() {
        manage_game(games[i].clone(), &mut games);
    }
    let mut n = 0;
    games.iter().for_each(|g| n += g.instances);
    println!("Result: {}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
