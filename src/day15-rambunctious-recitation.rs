use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;

fn game(mut seq : Vec<u64>, to: u64) -> u64 {
    seq.reserve(to as usize);

    let mut birth: HashMap<u64, u64> = HashMap::new();

    seq[0..seq.len()-1].iter().enumerate().for_each(|(i, &s)| {
        birth.insert(s, i as u64);
    });

    for turn in seq.len() as u64..to {
        let spoken = *seq.last().unwrap();
        let born = match birth.get(&spoken) {
            Some(b) => b + 1,
            None => turn,
        };
        seq.push(turn - born);
        birth.insert(spoken, turn-1);
    }

    *seq.last().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Provide input file");
    }

    let file = File::open(&args[1])
        .expect("Couldn't read input file");

    let mut line = String::new();
    io::BufReader::new(file).read_line(&mut line)
        .expect("Could not read first line");

    let seq : Vec<u64> = line.split(',')
        .map(|c| c.parse().unwrap() )
        .collect();

    println!("Part 1: {}", game(seq.clone(), 2020));
    println!("Part 2: {}", game(seq, 30_000_000));
}