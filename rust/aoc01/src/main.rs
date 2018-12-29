use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn compute(filepath: &String) -> i64 {
    let file = File::open(filepath).expect("Unable to open the file");
    let lines = BufReader::new(file).lines();
    lines.filter_map(|n| n.unwrap().parse::<i64>().ok()).sum()
}

fn compute_v2(filepath: &String) -> i64 {
    let file = File::open(filepath).expect("Unable to open the file");
    let lines : Vec<String> = BufReader::new(file).lines().map(|line| line.unwrap()).collect();
    let mut cline = 0;
    let mut cfreq = 0;
    let mut frequencies: HashSet<i64> = HashSet::new();
    loop {
        if frequencies.contains(&cfreq) {
            break;
        }
        frequencies.insert(cfreq);
        cfreq += lines[cline].parse::<i64>().unwrap();
        cline = (cline + 1) % lines.len();
    }
    return cfreq
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // The file must the first argument
    if args.len() == 1 {
        eprintln!("Please to set a file name");
        std::process::exit(1);
    }
    println!("The result for the first part is {}", compute(&args[1]));
    println!("The result is {}", compute_v2(&args[1]));
}
