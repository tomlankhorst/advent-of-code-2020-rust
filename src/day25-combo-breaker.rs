use std::env;

fn transform_step(subject : u64, mut value : u64) -> u64 {
    value = value * subject;
    value % 20201227
}

fn find_loop_size(public_key : u64) -> u64 {
    let subject : u64 = 7;
    let mut value : u64 = 1;
    let mut loop_size : u64 = 0;
    while value != public_key {
        value = transform_step(subject, value);
        loop_size = loop_size + 1;
    }
    loop_size
}

fn transform_n(subject : u64, n : u64) -> u64 {
    let mut value : u64 = 1;
    for _ in 0..n {
        value = transform_step(subject, value)
    }
    value
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Provide card and door public key");
    }

    let card_public_key = args[1].parse().expect("Could not parse card public key");
    let door_public_key = args[2].parse().expect("Could not parse door public key");

    let door_loop_size = find_loop_size(door_public_key);
    let encryption_key = transform_n(card_public_key, door_loop_size);

    println!("Part 1: {}", encryption_key);
}

#[cfg(test)]
mod tests {
    use crate::{find_loop_size, transform_n};

    #[test]
    fn part1_encryption_key() {
        let card_public_key =  5_764_801;
        let door_public_key = 17_807_724;

        let card_loop_size = find_loop_size(card_public_key);
        assert_eq!(card_loop_size, 8);
        let door_loop_size = find_loop_size(door_public_key);
        assert_eq!(door_loop_size, 11);

        let encryption_key = transform_n(card_public_key, door_loop_size);
        assert_eq!(encryption_key, transform_n(door_public_key, card_loop_size));
        assert_eq!(encryption_key, 14_897_079)
    }
}