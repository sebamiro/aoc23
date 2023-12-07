use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug)]
enum Type {
    FIVE,
    FOUR,
    FULL,
    THREE,
    TWOPAIR,
    ONEPAIR,
    NONE
}

#[derive(Debug)]
struct Hand {
    bid: u64,
    hand_type: Type,
    cards: Vec<u8>
}

fn new_cards_vec(s: &str) -> Vec<u8> {
    let mut cards: Vec<u8> = Vec::new();

    for c in s.chars() {
        let n: u8 = match c {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 0,
            'T' => 9,
            _ => c.to_digit(10).unwrap() as u8 - 1
        };
        cards.push(n);
    }
    cards
}

fn new_type(mut cards: Vec<u8>) -> Type {
    let mut count = 0;
    let mut max = 0;
    cards.iter().for_each(|x| {
        let c = cards.iter().filter(|z| *z == x).count();
        if *x != 0 && c > count {
            count = c;
            max = *x;
        }
    });
    cards.iter_mut().for_each(|x| if *x == 0 { *x = max });
    cards.sort();
    let mut last = 42;
    let mut three: bool = false;
    let mut pairs: u8 = 0;
    for n in cards.iter() {
        if *n == last {
            continue;
        }
        let count = cards.iter().filter(|x| *x == n).count();
        match count {
            5 => {
                eprintln!("Type: TWOPAIR");
                return Type::FIVE;
            },
            4 => {
                eprintln!("Type: TWOPAIR");
                return Type::FOUR;
            },
            3 => three = true,
            2 => pairs += 1,
            _ => ()
        }
        last = *n;
    }
    if pairs == 2 {
        eprintln!("Type: TWOPAIR");
        return Type::TWOPAIR;
    }
    if three && pairs == 1 {
        eprintln!("Type: FULL");
        return Type::FULL;
    }
    if three {
        eprintln!("Type: THREE");
        return Type::THREE;
    }
    if pairs == 1 {
        eprintln!("Type: ONEPAIR");
        return Type::ONEPAIR;
    }
    eprintln!("Type: NONE");
    Type::NONE
}

fn new_hand(s: String) -> Hand {
    let (cards, bid) = s.split_once(' ').unwrap();
    eprintln!("Cards: {}, Bid: {}", bid, cards);
    let cards = new_cards_vec(cards);
    let hand_type = new_type(cards.clone());
    Hand{ bid: bid.parse::<u64>().unwrap(), cards, hand_type }
}

fn type_to_int(h: &Type) -> u8 {
    match h {
        Type::FIVE => 7,
        Type::FOUR => 6,
        Type::FULL => 5,
        Type::THREE => 4,
        Type::TWOPAIR => 3,
        Type::ONEPAIR => 2,
        Type::NONE => 1
    }
}

fn cmp_hand(h1: &Hand, h2: &Hand) -> Ordering {
    let h1_type = type_to_int(&h1.hand_type);
    let h2_type = type_to_int(&h2.hand_type);

    if h1_type == h2_type {
        return h1.cards.cmp(&h2.cards);
    }
    h1_type.cmp(&h2_type)
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    let mut hands: Vec<Hand> = Vec::new();
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    hands.push(new_hand(s));
                }
            }
        }
    }
    hands.sort_by(cmp_hand);
    let mut n = 0;
    for (i, h) in hands.iter().enumerate() {
        eprintln!("{}: {:?}", i, h);
        n += (i + 1) * h.bid as usize;
    }
    println!("Result: {}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
