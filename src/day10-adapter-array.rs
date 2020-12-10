use std::collections::HashMap;
use std::cmp::max;

mod tools;

fn main() {
    let lines = match tools::input_reader() {
        Some(l) => l,
        None => panic!(),
    };

    let mut nums : Vec<u64> = lines
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    nums.push(0);
    nums.sort();
    nums.push(nums.last().unwrap()+3);

    let steps : Vec<u64> = nums
        .windows(2)
        .map(|v| v[1] - v[0])
        .collect();

    let one_diffs = steps.iter().filter(|&&v| v == 1).count();
    let three_diffs = steps.iter().filter(|&&v| v == 3).count();

    println!("Part 1: one {}, three {}, mult: {}", one_diffs, three_diffs, one_diffs * three_diffs);

    let mut reachable : HashMap<u64, Vec<u64>> = HashMap::new();

    reachable.reserve(nums.len());

    for i in 1..nums.len() {
        let mut from : Vec<u64> = Vec::new();
        from.reserve(3);
        let adapter = nums[i] as i64;
        let start = max(0, i as isize -3) as usize;
        for x in start..=i {
            if nums[x] as i64 >= adapter - 3 {
                from.push(nums[x]);
            }
        }
        reachable.insert(adapter as u64, from);
    }
        //
        // nums
        // .windows(4)
        // .map(|v| {
        //     let mut from : Vec<u64> = Vec::new();
        //     from.reserve(3);
        //     let least : i64 = (*v.last().unwrap()) as i64 - 3;
        //     for i in 0..3 {
        //         if v[i] as i64 >= least {
        //             from.push(v[i]);
        //         }
        //     }
        //
        //     (v[3], from)
        // } )
        // .collect();

    let mut paths: HashMap<u64, u64> = HashMap::new();

    fn traverse(from : u64, to : u64, reachable : &HashMap<u64, Vec<u64>>, mut paths: &mut HashMap<u64, u64>) -> u64 {
        match paths.get(&to) {
            Some(&v) => v,
            None => {
                let ways = match reachable.get(&to) {
                    Some(reaches) => {
                        let mut ways = 0;
                        for &reach in reaches {
                            if reach == from {
                                ways = ways + 1;
                            } else {
                                ways = ways + traverse(from, reach, &reachable, & mut paths);
                            }
                        }
                        ways
                    },
                    None => 0
                };
                paths.insert(to, ways);
                ways
            },
        }
    }

    let opts = traverse(0, *nums.last().unwrap(), &reachable, & mut paths);

    println!("Part 2: {}", opts)

}