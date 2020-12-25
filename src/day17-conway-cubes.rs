use std::collections::{HashSet, HashMap};
use std::{env, io};
use std::fs::File;
use std::io::Read;

type Coord = (i8,i8,i8,i8);
type World = HashSet<Coord>;
type Neighbors = HashMap<Coord, u8>;

fn read(input : &str) -> World {
    input
        .lines()
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x,c)|{
                if c == '#' {
                    Some((x as i8,y as i8,0,0))
                } else {
                    None
                }
            })
        }).collect()
}

fn evolve(mut world : World, dim : i32, evolutions : i32) -> World {
    for _i in 0..evolutions {
        let mut neighbors : Neighbors = Neighbors::new();
        for (x,y,z,w) in world.iter() {
            for dx in -1..=1 as i8 {
                for dy in -1..=1 as i8 {
                    for dz in -1..=1 as i8 {
                        for dw in if dim == 4 {-1..=1 as i8} else {0..=0} {
                            if dx==0 && dy==0 && dz==0 && dw==0 {
                                continue;
                            }
                            let coord = (x+&dx,y+&dy,z+&dz,w+&dw);
                            neighbors.insert(coord, neighbors.get(&coord).unwrap_or(&0) + 1);
                        }
                    }
                }
            }
        }
        world = neighbors.iter().filter_map(|(&coord, &adjacent)| {
            if adjacent == 3 || (adjacent == 2 && world.contains(&coord)) {
                Some(coord)
            } else {
                None
            }
        }).collect()
    }
    world
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

    let start = read(&input);
    let mut world = start.clone();
    world = evolve(world, 3, 6);

    println!("Part 1: {}", world.len());

    world = start;
    world = evolve(world, 4, 6);
    println!("Part 2: {}", world.len());
}

#[cfg(test)]
mod tests {
    use crate::{read, evolve};

    #[test]
    fn part1_3d_world_example() {
        let mut world = read(".#.\n..#\n###\n");
        world = evolve(world, 3, 6);
        assert_eq!(world.len(), 112);
    }

    #[test]
    fn part2_4d_world_example() {
        let mut world = read(".#.\n..#\n###\n");
        world = evolve(world, 4, 6);
        assert_eq!(world.len(), 848);
    }
}
