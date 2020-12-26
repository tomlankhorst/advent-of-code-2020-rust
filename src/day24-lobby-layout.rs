use std::collections::HashMap;
use std::{env, io};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    East, NorthEast, NorthWest, West, SouthWest, SouthEast,
}

fn coord(d : Direction) -> (i32, i32) {
    match d {
        Direction::East => (2, 0),
        Direction::NorthEast => (1, 1),
        Direction::NorthWest => (-1, 1),
        Direction::West => (-2, 0),
        Direction::SouthWest => (-1, -1),
        Direction::SouthEast => (1, -1),
    }
}

type Instructions = Vec<Vec<Direction>>;

fn read_directions(input : &str) -> Instructions {
    input.lines().into_iter()
        .map(|l| {
            let mut it = l.chars().into_iter();
            let mut directions : Vec<Direction> = Vec::new();
            loop {
                let c = match it.next() {
                    Some(c) => c,
                    None => break,
                };
                let dir = match c {
                    'e' => Direction::East,
                    'w' => Direction::West,
                    _ => {
                        match (c, it.next().expect("Expected another char after n or s")) {
                            ('n', 'e') => Direction::NorthEast,
                            ('n', 'w') => Direction::NorthWest,
                            ('s', 'e') => Direction::SouthEast,
                            ('s', 'w') => Direction::SouthWest,
                            _ => panic!("Unexpected!"),
                        }
                    }
                };
                directions.push(dir);
            }
            directions
        })
        .collect()
}

type Coord = (i32,i32);
type Floor = HashMap<Coord, bool>;

fn do_floor(ins : Instructions) -> Floor {
    let mut floor = Floor::default();
    ins.iter().map(|i| {
        i.iter().fold((0, 0), |c, &d| {
            let dc = coord(d);
            (c.0 + dc.0, c.1 + dc.1)
        })
    }).for_each(|c| {
        floor.insert(c, !*floor.get(&c).unwrap_or(&false));
    });
    floor
}

fn floor_evolve(mut floor : Floor) -> Floor {
    let mut neighbors : HashMap<Coord, i32> = HashMap::new();

    floor.iter().filter(|(_,&v)| v).for_each(|(&c,_)| {
        for dir in &[Direction::East, Direction::NorthEast, Direction::NorthWest, Direction::West, Direction::SouthWest, Direction::SouthEast] {
            let dc = coord(*dir);
            let cd = (c.0 + dc.0, c.1 + dc.1);
            neighbors.insert(cd, neighbors.get(&cd).unwrap_or(&0) + 1);
        }
    });

    floor.iter_mut()
        .filter(|(&c,& mut v)| v && !neighbors.contains_key(&c))
        .for_each(|(_, v)| *v = false );

    neighbors.iter().for_each(|(&c, &n)| {
        let is_black = *floor.get(&c).unwrap_or(&false);
        let flip = (is_black && (n == 0 || n > 2)) || (!is_black && n == 2);
        if flip {
            floor.insert(c, !*floor.get(&c).unwrap_or(&false));
        }
    });

    floor
}

fn floor_count_black(floor : &Floor) -> usize {
    floor.iter().filter(|(_,&v)| v).count()
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

    let directions = read_directions(&input);

    let mut floor = do_floor(directions);

    println!("Part 1: {}", floor_count_black(&floor));

    for _ in 0..100 {
        floor = floor_evolve(floor);
    }

    println!("Part 2: {}", floor_count_black(&floor));
}

#[cfg(test)]
mod tests {
    use crate::{read_directions, Instructions, Direction, do_floor, floor_evolve, floor_count_black};

    fn example_input() -> Instructions {
    read_directions(r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#)
    }

    #[test]
    fn test_read_directions() {
        let dirs = example_input();
        assert_eq!(dirs[1][1], Direction::East);
    }

    #[test]
    fn do_example_floor() {
        let floor = do_floor(example_input());
        assert_eq!(floor_count_black(&floor), 10);
    }

    #[test]
    fn do_floor_evolve() {
        let mut floor = do_floor(example_input());

        floor = floor_evolve(floor);

        assert_eq!(floor_count_black(&floor), 15);
        for _ in 1..100 {
            floor = floor_evolve(floor)
        }

        assert_eq!(floor_count_black(&floor), 2208);
    }

}