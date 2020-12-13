use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Lines, BufReader};
use std::mem::swap;

#[derive(Default,Debug,Copy,Clone)]
struct Coord ( i32, i32 );

impl Coord {
    fn rot(mut self, mut degrees : i32) -> Self {
        degrees = degrees % 360;
        let rots = (4 + degrees/90) % 4;
        for _i in 0..rots {
            swap(& mut self.0, & mut self.1);
            self.1 = -self.1;
        }
        self
    }
    fn mul(mut self, times : i32) -> Self {
        self.0 = self.0 * times;
        self.1 = self.1 * times;
        self
    }
    fn add(mut self, rhs : Coord) -> Self {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
        self
    }
}

#[derive(Debug,Copy,Clone)]
enum Direction { North, East, South, West }

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Turn { Left, Right }

#[derive(Debug)]
enum Action { Direction(Direction), Turn(Turn), Move }

impl Direction {
    fn direction_coord(&self) -> Coord {
        match self {
            Direction::North => Coord (0, 1),
            Direction::East => Coord (1, 0),
            Direction::South => Coord (0, -1),
            Direction::West => Coord (-1, 0),
        }
    }
}

impl Default for Direction {
    fn default() -> Self { Direction::East }
}

#[derive(Default,Debug,Copy,Clone)]
struct Ship {
    pos : Coord,
    dir : Direction,
    wayp : Coord,
}

impl Ship {
    fn move_by(&mut self, d : &Direction, steps : i32) {
        self.pos = self.pos.add(d.direction_coord().mul(steps));
    }
    fn move_waypoint(&mut self, d : &Direction, steps : i32) {
        self.wayp = self.wayp.add(d.direction_coord().mul(steps));
    }
    fn follow_waypoint(&mut self, steps : i32) {
        self.pos = self.pos.add(self.wayp.mul(steps));
    }
    fn turn(&mut self, t : &Turn, degrees : i32) {
        for _i in 0..(degrees/90) {
            self.dir = match self.dir {
                Direction::North => if t == &Turn::Left { Direction::West } else { Direction::East },
                Direction::East => if t == &Turn::Left { Direction::North } else { Direction::South },
                Direction::South => if t == &Turn::Left { Direction::East } else { Direction::West },
                Direction::West => if t == &Turn::Left { Direction::South } else { Direction::North },
            };
        }
    }
    fn turn_waypoint(&mut self, t : &Turn, mut degrees: i32) {
        degrees = match t {
            Turn::Left => -degrees,
            _ => degrees
        };
        self.wayp = self.wayp.rot(degrees);
    }
    fn manhattan_distance(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs()
    }
}

#[derive(Debug)]
struct Instruction {
    action : Action,
    value : i32
}

type Instructions = Vec::<Instruction>;

fn read_instructions(lines : Lines<BufReader<File>>) -> Instructions {
    lines
        .map(|l| {
            let l = l.unwrap();
            Instruction {
                action: match l.chars().next().unwrap() {
                    'N' => Action::Direction(Direction::North),
                    'E' => Action::Direction(Direction::East),
                    'S' => Action::Direction(Direction::South),
                    'W' => Action::Direction(Direction::West),
                    'L' => Action::Turn(Turn::Left),
                    'R' => Action::Turn(Turn::Right),
                    _ => Action::Move,
                },
                value: l[1..].parse().unwrap(),
            }
        })
        .collect()
}

fn ship_movement_strategy (mut ship : Ship, instruction : &Instruction) -> Ship {
    match &instruction.action {
        Action::Move => ship.move_by(&ship.dir.clone(), instruction.value),
        Action::Direction(dir) => ship.move_by(dir, instruction.value),
        Action::Turn(turn) => ship.turn(turn, instruction.value),
    }
    ship
}

fn ship_waypoint_strategy (mut ship : Ship, instruction : &Instruction) -> Ship {
    match &instruction.action {
        Action::Move => ship.follow_waypoint(instruction.value),
        Action::Direction(dir) => ship.move_waypoint(dir, instruction.value),
        Action::Turn(turn) => ship.turn_waypoint(turn, instruction.value),
    }
    ship
}

fn navigate(mut ship: Ship, instructions : &Instructions, strategy : fn(Ship, &Instruction) -> Ship) -> Ship {
    instructions.iter()
        .for_each(|i| ship = strategy(ship, i));
    ship
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Provide input file");
    }

    let file = File::open(&args[1])
        .expect("Couldn't read input file");

    let instructions = read_instructions(
        io::BufReader::new(file).lines()
    );

    let ship = navigate(Ship::default(), &instructions, ship_movement_strategy);
    println!("Part 1: {:?}, Manhattan: {}", ship, ship.manhattan_distance());

    let mut ship = Ship::default();
    ship.wayp = Coord(10, 1);
    ship = navigate(ship, &instructions, ship_waypoint_strategy);
    println!("Part 2: {:?}, Manhattan: {}", ship, ship.manhattan_distance());
}