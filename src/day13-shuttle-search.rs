use std::{env, io};
use std::fs::File;
use std::io::BufRead;

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> i64 {
    let (g, x, _) = egcd(x, n);
    if g != 1 {
        panic!("Not co-prime")
    }
    (x % n + n) % n
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> i64 {
    let n : i64 = modulii.iter().product();

    residues.iter().zip(modulii).fold(0, |a, (&r, &m)| {
        let p = (r as i128 * mod_inv(n/m, m) as i128) as i64;
        a + p * (n/m)
    }) % n
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Provide input file");
    }

    let file = File::open(&args[1])
        .expect("Couldn't read input file");

    let mut lines = io::BufReader::new(file).lines();

    let earliest : i64 = lines.next().unwrap().unwrap().parse().unwrap();

    enum Schedule { Bus(i64), None };
    type S = Schedule;

    let schedule : Vec<S> = lines.next().unwrap().unwrap()
        .split(',')
        .map(|e| match e {
            "x" => S::None,
            _ => S::Bus(e.parse::<i64>().unwrap()),
        })
        .collect();

    let upcomping : Vec<i64> = schedule.iter().map(|s| match s {
        S::None => std::i64::MAX,
        S::Bus(freq) => freq - earliest % freq,
    }).collect();

    let quickest_schedule = upcomping.iter().enumerate()
        .min_by(|&(_, a), &(_, b)| a.cmp(b)).unwrap();

    if let S::Bus(quickest_bus) = schedule[quickest_schedule.0] {
        println!("Part 1: from {} on, take bus {}, wait {}: {}", earliest, quickest_bus, quickest_schedule.1, quickest_schedule.1 * quickest_bus);
    } else {
        panic!("Part 1: found no answer");
    }

    let residues : Vec<i64> = schedule.iter().enumerate().map(|(i, s)| match s {
        S::None => 0,
        S::Bus(bus) => (bus - i as i64),
    }).collect();

    let modulii : Vec<i64> = schedule.iter().map(|s| match s {
        S::None => 1,
        S::Bus(bus) => *bus,
    }).collect();

    let sol = chinese_remainder(&residues, &modulii);
    println!("Part 2: {}", sol);
}