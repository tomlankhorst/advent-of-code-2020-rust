use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashMap;
use std::mem::swap;

#[derive(Default, Clone)]
struct Mask {
    value : String,
}

impl Mask {
    fn mask_for(&self, x : char) -> u64 {
        self.value.chars().fold(0, |a, c| {
            if c == x {
                a << 1 | 1
            } else {
                a << 1
            }
        })
    }
}

#[derive(Copy, Clone)]
struct Mem {
    addr: u64,
    val: u64,
}

enum Instruction {
    Mask(Mask),
    Mem(Mem),
    Nop,
}

type Instructions = Vec::<Instruction>;

#[derive(Default)]
struct VM {
    mask : Mask,
    mem : HashMap<u64, u64>,
}

impl VM {
    fn execute(&mut self, instructions : &Instructions, mem_strategy : fn(&mut VM, Mem)) {
        for i in instructions {
            match i {
                Instruction::Mask(mask) => self.mask = mask.clone(),
                Instruction::Mem(mem) => mem_strategy(self, mem.clone()),
                _ => {},
            }
        }
    }
    fn set(&mut self, mem : Mem) {
        let mut val = mem.val;
        val = (val | self.mask.mask_for('1')) & !self.mask.mask_for('0');
        self.mem.insert(mem.addr, val);
    }
    fn mem_access_buff(&mut self, mem : Mem) {
        match self.mask.mask_for('X') {
            0 => { self.mem.insert(mem.addr | self.mask.mask_for('1'), mem.val); },
            floating_mask => {
                let msb_pos = (floating_mask.leading_zeros() - (64-36)) as usize;
                let msb_mask = 1 << (35 - msb_pos);

                let mut mask = self.mask.clone();
                mask.value = mask.value.replacen("X", "0", 1);

                swap(&mut self.mask, &mut mask);
                self.mem_access_buff(Mem { addr: mem.addr | msb_mask, val: mem.val });
                self.mem_access_buff(Mem { addr: mem.addr & !msb_mask, val: mem.val });
                swap(&mut self.mask, &mut mask);
            }
        };
    }
    fn sum(&self) -> u64 {
        self.mem.iter().fold(0, |a,(_,v)| a + v)
    }
}

fn read_instructions(lines : Lines<BufReader<File>>) -> Instructions {
    lines
        .map(|l| l.unwrap())
        .map(|l| match &l[0..3] {
            "mem" => {
                let bp = l.find(']').unwrap();
                Instruction::Mem(Mem{
                    addr: l[4..bp].parse().unwrap(),
                    val: l[bp+4..].parse().unwrap(),
                })
            },
            "mas" => Instruction::Mask(Mask{ value: l[7..].parse().unwrap() }),
            _ => Instruction::Nop,
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Provide input file");
    }

    let file = File::open(&args[1])
        .expect("Couldn't read input file");

    let lines = io::BufReader::new(file).lines();

    let instructions : Instructions = read_instructions(lines);

    let mut vm = VM::default();

    vm.execute(&instructions, VM::set);

    println!("Part 1: {}", vm.sum());

    vm = VM::default();
    vm.execute(&instructions, VM::mem_access_buff);

    println!("Part 2: {}", vm.sum());
}