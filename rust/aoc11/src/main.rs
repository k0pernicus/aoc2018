use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {

    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords : Vec<&str> = s.trim_matches(|p| p == '(' || p == ')').split(',').collect();
        let x = coords[0].parse::<isize>()?;
        let y = coords[1].parse::<isize>()?;

        Ok(Point { x: x, y: y})
    }

}



fn main() {
    // Process the file
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Please to set a file name");
        std::process::exit(1);
    }
    let f_pointer = File::open(&args[1]).expect(
        "Unable to open
                                    the given file",
    );
    let f_lines: Vec<String> = BufReader::new(f_pointer)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let points: Vec<Point> = f_lines
        .iter()
        .map(|l| Point::from_str(l).ok().unwrap())
        .collect();
}
