extern crate itertools;

use itertools::Itertools;

use std::collections::BTreeMap;
use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn get_combination(lines: &[String]) -> u64 {
    let mut count_2_value = 0;
    let mut count_3_value = 0;
    for line in lines {
        let mut count = BTreeMap::new();
        for c in line.chars() {
            *count.entry(c).or_insert(0) += 1;
        }
        // Get the occurences for each character
        let occurences : Vec<u64> = count.values().cloned().collect();
        // Remove all duplicates
        let unique_occurences : Vec<u64> = occurences.into_iter().unique().collect();
        // Count the number of 2 and 3 values
        unique_occurences.iter().foreach(|i| if i == &2 { count_2_value += 1 } else if i == &3 { count_3_value += 1 });
    }
    return count_2_value * count_3_value
}

fn get_common_chars(lines: &[String]) -> String {
    for i in 0..lines.len() {
        let c_string = &lines[i];
        for o_lines in lines[i..].iter() {
            let matching = c_string.chars()
                                   .zip(o_lines.chars()).filter(|&(a, b)| a == b);
            if matching.clone().count() == c_string.len() - 1 {
                println!("Found interesting strings: \"{}\" / \"{}\"", c_string, o_lines);
                println!("> Common base is \"{}\"", matching.collect::<Vec<(char, char)>>()
                                                      .iter()
                                                      .fold("".to_string(), |mut s, c| {s.push(c.0); s}));
            }
        }
    }
    return String::new()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // The file must the first argument
    if args.len() == 1 {
        eprintln!("Please to set a file name");
        std::process::exit(1);
    }
    let f_pointer = File::open(&args[1]).expect("Unable to open the given file");
    let f_lines: Vec<String> = BufReader::new(f_pointer).lines().map(|line| line.unwrap()).collect();
    println!("The result for the first part is {}", get_combination(&f_lines[..]));
    get_common_chars(&f_lines[..]);
}
