use std::{env,io::{self,BufRead},fs::File};
use std::io::{BufReader, Lines};

pub fn input_reader() -> Option<Lines<BufReader<File>>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Provide day1 file");
        return None
    }

    let file = File::open(&args[1])
        .expect("Couldn't read day1");

    Some(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
