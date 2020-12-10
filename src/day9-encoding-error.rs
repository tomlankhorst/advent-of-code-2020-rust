mod tools;

fn main() {
    let lines = match tools::input_reader() {
        Some(l) => l,
        None => panic!(),
    };

    let nums : Vec<u64> = lines
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let preamble = 25;

    let mut num = 0;
    for i in preamble..nums.len() {
        num = nums[i];
        let mut found = false;
        for j in i-preamble..i {
            for k in j..i {
                if nums[j] + nums[k] == num {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            println!("Part 1: {}", num);
            break;
        }
    }

    for i in 0..nums.len()-1 {
        for j in i+2..nums.len() {
            let s : u64 = nums[i..j].iter().sum();
            if s == num {
                let range = &nums[i..j];
                let sum = range.iter().min().unwrap() + range.iter().max().unwrap();
                println!("Part 2: {}", sum);
            } else if s > num {
                break;
            }
        }
    }

}