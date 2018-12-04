#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result;
use std::str::FromStr;

use regex::Regex;

const HEIGHT: usize = 1000;
const WIDTH: usize = 1000;

#[derive(Debug, PartialEq)]
struct Area {
    id: usize,
    from_left: usize,
    from_top: usize,
    width: usize,
    height: usize,
}

type Result<T> = result::Result<T, Box<Error>>;

impl FromStr for Area {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Area> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                \#
                (?P<id>[0-9]+)
                \s+@\s+
                (?P<fl>[0-9]+),(?P<ft>[0-9]+):
                \s+
                (?P<w>[0-9]+)x(?P<h>[0-9]+)
            ").unwrap();
        }

        let caps = match RE.captures(s) {
            None => panic!("Error capturing the stream"),
            Some(caps) => caps,
        };
        Ok(Area {
            id: caps["id"].parse()?,
            from_left: caps["fl"].parse()?,
            from_top: caps["ft"].parse()?,
            width: caps["w"].parse()?,
            height: caps["h"].parse()?,
        })
    }
}

fn part_1(lines: Vec<String>, fabric: &mut[[usize; WIDTH]; HEIGHT]) -> usize {
    for line in lines {
        let area = Area::from_str(&line).unwrap();
        for i in area.from_top..(area.from_top + area.height) {
            for j in area.from_left..(area.from_left + area.width) {
                fabric[i][j] += 1;
            }
        }
    }
    fabric.iter()
          .map(|l| l.into_iter().filter(|&&i| i > 1).count())
          .collect::<Vec<usize>>()
          .iter()
          .fold(0usize, |mut s, i| {s += i; s})
}

fn main() {
    // Process the file
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Please to set a file name");
        std::process::exit(1);
    }
    let f_pointer = File::open(&args[1]).expect("Unable to open the given file");
    let f_lines: Vec<String> = BufReader::new(f_pointer).lines().map(|line| line.unwrap()).collect();
    // Initialize the fabric
    let mut fabric = [[0usize; WIDTH]; HEIGHT];
    // Compute the squares
    let nb_occupied_squares = part_1(f_lines, &mut fabric);
    println!("Nb occupied squares is {}", nb_occupied_squares);
}
