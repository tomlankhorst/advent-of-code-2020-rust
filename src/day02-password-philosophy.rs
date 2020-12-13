mod tools;

fn main() {
    let lines = match tools::input_reader() {
        Some(l) => l,
        None => panic!(),
    };

    let mut valid : usize = 0;

    for line in lines {
        if let Ok(line) = line {
            let parts : Vec<&str> = line.split(|c| c == ' ' || c == ':' || c == '-').collect();
            if let [from, to, ch, _, pass] = &parts[..] {
                let from : usize = from.parse().unwrap();
                let to : usize = to.parse().unwrap();
                let ch = ch.chars().next().unwrap();
                let count = pass.chars().filter(|&c| c == ch).count();
                if count >= from && count <= to {
                    valid = valid+1;
                }
            }
        }
    }

    println!("Part 1: {}", valid)
}