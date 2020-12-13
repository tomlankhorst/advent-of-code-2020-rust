mod tools;

#[derive(Clone)]
enum Operation {
    Nop,
    Jmp,
    Acc,
}

#[derive(Clone)]
struct Instruction {
    op : Operation,
    val : isize,
}

#[derive(Default, Debug)]
struct Machine {
    pc : usize,
    r0 : usize,
}

#[derive(Debug,PartialEq)]
enum Exit { End, Loop }

#[derive(Debug)]
struct ExecResult {
    m : Machine,
    exit : Exit,
}

type Program = Vec<Instruction>;

fn execute(p : &Program, m : Option<Machine>) -> ExecResult {

    let mut m = m.unwrap_or(Machine::default());

    let p_len = p.len();

    let exit;

    let mut visited : Vec<bool> = Vec::new();
    visited.resize(p_len, false);

    loop {
        if m.pc >= p_len {
            exit = Exit::End;
            break;
        }

        if visited[m.pc] {
            exit = Exit::Loop;
            break;
        }

        visited[m.pc] = true;

        let ins = &p[m.pc];
        match ins.op {
            Operation::Acc => ( m.r0 = (m.r0 as isize + ins.val) as usize ),
            Operation::Jmp => {
                m.pc = (m.pc as isize + ins.val) as usize;
                continue;
            },
            _ => {}
        }
        m.pc = m.pc + 1;
    }

    ExecResult { m, exit }
}

fn main() {
    let lines = match tools::input_reader() {
        Some(l) => l,
        None => panic!(),
    };

    let mut program: Program = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let op_str = &line[0..3];
            let val_str = &line[4..];
            program.push(Instruction {
                op: match &op_str[..] {
                    "jmp" => Operation::Jmp,
                    "acc" => Operation::Acc,
                    _ => Operation::Nop,
                },
                val: val_str.trim().parse().unwrap(),
            });
        }
    }

    let machine = execute(&program, None);
    println!("{:?}", machine);

    for i in 0..program.len() {
        let mut copy = program.clone();
        let ins = & mut copy[i];
        match ins.op {
            Operation::Jmp => {
                ins.op = Operation::Nop;
            },
            Operation::Nop => {
                ins.op = Operation::Jmp;
            },
            _ => continue
        }
        let res = execute(&copy, None);
        if res.exit == Exit::End {
            println!("Part 2: Changed instruction {}, {:?}", i, machine);
            break
        }
    }
}
