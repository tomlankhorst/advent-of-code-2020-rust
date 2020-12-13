mod tools;

fn main() {
    let lines = match tools::input_reader() {
        Some(l) => l,
        None => panic!(),
    };

    let mut nums = Vec::<u32>::new();

    for line in lines {
        if let Ok(num) = line {
            let num : u32 = num.trim().parse().expect("Couldn't parse");
            nums.push(num);
        }
    }

    for i in 1..nums.len() {
        for j in i..nums.len() {
            let ni = nums[i];
            let nj = nums[j];
            if ni + nj == 2020 {
                println!("part 1: {}", ni * nj);
            }
            for k in j..nums.len() {
                let nk = nums[k];
                if ni + nj + nk == 2020 {
                    println!("part 2: {}", ni * nj * nk);
                }
            }
        }
    }
}
