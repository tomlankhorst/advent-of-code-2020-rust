#![feature(iter_map_while)]

use std::collections::{HashSet, VecDeque};
use std::str::Lines;
use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

type Deck = VecDeque<u8>;
type Decks = (Deck, Deck);

#[derive(Debug,PartialEq)]
enum Player { One, Two }

fn read_deck(lines : & mut Lines) -> Deck {
    lines.map_while(|l| {
        if l.is_empty() {
            None
        } else {
            Some(l.parse::<u8>().unwrap())
        }
    })
        .collect()
}

fn read(input : &str) -> Decks {
    let mut decks : Decks = (Default::default(), Default::default());

    let lines = & mut input.lines();

    lines.next();
    decks.0 = read_deck(lines);
    lines.next();
    decks.1 = read_deck(lines);

    decks
}

fn winner(decks : &Decks) -> (Player, u32, Deck) {
    let mut result = if decks.0.is_empty() { (Player::Two, 0, decks.1.clone()) } else { (Player::One, 0, decks.0.clone()) };
    result.1 = result.2
        .iter()
        .rev()
        .enumerate()
        .fold(0, |a, (i,&c)| a + (i as u32 + 1) * c as u32);
    result
}

fn combat(mut decks : Decks) -> (Player, u32, Deck) {
    while !decks.0.is_empty() && !decks.1.is_empty() {
        let c0 = decks.0.pop_front().unwrap();
        let c1 = decks.1.pop_front().unwrap();
        if c0 > c1 {
            decks.0.push_back(c0);
            decks.0.push_back(c1);
        } else {
            decks.1.push_back(c1);
            decks.1.push_back(c0);
        }
    }
    winner(&decks)
}

fn recursive_combat(mut decks : Decks) -> (Player, u32, Deck) {
    let mut seen : HashSet<u64> = HashSet::new();
    while !decks.0.is_empty() && !decks.1.is_empty() {
        let mut hasher = DefaultHasher::new();
        decks.hash(&mut hasher);
        let decks_hash = hasher.finish();
        let wins = if !seen.contains(&decks_hash) {
            seen.insert(decks_hash);
            let c0 = *decks.0.front().unwrap() as usize;
            let c1 = *decks.1.front().unwrap() as usize;
            if c0 < decks.0.len() && c1 < decks.1.len() {
                let mut decks = decks.clone();
                decks.0.pop_front();
                decks.0.truncate(c0);
                decks.1.pop_front();
                decks.1.truncate(c1);
                recursive_combat(decks).0
            } else if c0 > c1 {
                Player::One
            } else {
                Player::Two
            }
        } else {
            // seen, player one wins. score not important in subround so just clear the second deck
            decks.1.clear();
            break;
        };

        let c0 = decks.0.pop_front().unwrap();
        let c1 = decks.1.pop_front().unwrap();
        if wins == Player::One {
            decks.0.push_back(c0);
            decks.0.push_back(c1);
        } else {
            decks.1.push_back(c1);
            decks.1.push_back(c0);
        }
    }
    winner(&decks)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Provide input file");
    }

    let file = File::open(&args[1])
        .expect("Couldn't read input file");

    let mut input = String::new();
    io::BufReader::new(file).read_to_string(&mut input).expect("Could not read file!");

    let starting = read(&input);

    println!("Part 1: {}", combat(starting.clone()).1);
    println!("Part 2: {}", recursive_combat(starting).1);
}

#[cfg(test)]
mod tests {
    use crate::{read, combat, Player, Decks, recursive_combat};

    fn example_deck() -> Decks {
        read(r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
"#)
    }

    #[test]
    fn part1_combat() {
        let decks = example_deck();
        assert_eq!(decks.0.front().unwrap(), &9);
        assert_eq!(decks.1.back().unwrap(), &10);
        let res = combat(decks);
        assert_eq!(res.0, Player::Two);
        assert_eq!(res.1, 306);
    }

    #[test]
    fn part2_recursive_combat() {
        let decks = example_deck();
        let res = recursive_combat(decks);
        assert_eq!(res.0, Player::Two);
        assert_eq!(res.1, 291);
    }
}
