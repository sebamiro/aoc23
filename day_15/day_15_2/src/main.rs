use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, process};

#[derive(Debug, Eq)]
struct Pair {
    k: String,
    v: u32
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.k == other.k
    }
}

fn hash(n: u32) -> u32 {
    (n * 17) % 256
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Input file missing");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(args[1].clone()) {
        let mut n = 0;
        let mut hm: BTreeMap<u32, Vec<Pair>> = BTreeMap::new();
        for line in lines {
            if let Ok(s) = line {
                if s.len() > 0 {
                    let steps = s.split(',');
                    for s in steps {
                        let (key, sign) = s.split_at(s.find(|c| c == '-' || c == '=').unwrap());
                        let (sign, num) = sign.split_at(1);
                        let mut hash_key = 0;
                        for c in key.bytes() {
                            hash_key = hash(hash_key + c as u32);
                        }
                        if let Some(h) = hm.get_mut(&hash_key) {
                            if sign == "-" {
                                if let Some(i) = h.iter().position(|x| x.k == key) {
                                    h.remove(i);
                                }
                            } else {
                                if let Some(i) = h.iter_mut().find(|x| x.k == key) {
                                    i.v = num.parse().unwrap();
                                } else {
                                    h.push(Pair{ k: key.to_string(), v: num.parse().unwrap()});
                                }
                            }
                        } else {
                            if sign == "=" {
                                hm.insert(hash_key, vec!(Pair{k: key.to_string(), v: num.parse().unwrap()}));
                            }
                        }
                    }
                }
            }
        }
        eprintln!("{:?}", hm);
        for (k, v) in hm.iter() {
            for (i, v) in v.iter().enumerate() {
                eprintln!("{} * {} * {}", k + 1, i + 1, v.v);
                n += (k + 1) * (i as u32 + 1) * v.v;
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
