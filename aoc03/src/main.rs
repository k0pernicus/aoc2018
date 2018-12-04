#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashSet;
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

fn part_1(lines: Vec<String>) -> usize {
    // Initialize the fabric
    let mut fabric = [[0usize; WIDTH]; HEIGHT];
    // Fill it
    for line in lines {
        let area = Area::from_str(&line).unwrap();
        for i in area.from_top..(area.from_top + area.height) {
            for j in area.from_left..(area.from_left + area.width) {
                fabric[i][j] += 1;
            }
        }
    }
    // Search for the number of overlapping areas
    fabric.iter()
          .map(|l| l.into_iter().filter(|&&i| i > 1).count())
          .collect::<Vec<usize>>()
          .iter()
          .fold(0usize, |mut s, i| {s += i; s})
}

fn part_2(lines: Vec<String>) -> Option<usize> {
    // Initialize a Vec<Vec<Vec<usize>>>, where usize is the
    // id of an area
    // The goal here is to find a single ID that corresponds
    // to the non-overlapped one
    let mut fabric: Vec<Vec<Vec<usize>>> = Vec::with_capacity(HEIGHT);
    for i in 0..HEIGHT {
        fabric.push(Vec::with_capacity(WIDTH));
        for _ in 0..WIDTH {
            fabric[i].push(Vec::new());
        }
    }
    // original_ids will contains all the box IDs (as, maybe, the ids are not consecutive...)
    let mut original_ids : HashSet<usize> = HashSet::new();
    // overlapped_ids will contains all the box IDs that are overlapping somewhere
    let mut overlapped_ids : HashSet<usize> = HashSet::new();
    // Fill the structure
    for line in lines {
        let area = Area::from_str(&line).unwrap();
        original_ids.insert(area.id);
        for i in area.from_top..(area.from_top + area.height) {
            for j in area.from_left..(area.from_left + area.width) {
                fabric[i][j].push(area.id);
            }
        }
    }
    // Search for overlapped boxes
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if fabric[i][j].len() > 1 {
                fabric[i][j].iter().for_each(|x| { let _ = overlapped_ids.insert(*x); });
            }
        }
    }
    // Compute the difference
    let ids_difference = original_ids.difference(&overlapped_ids).collect::<Vec<&usize>>();
    // Check the error
    if ids_difference.len() != 1 {
        return None
    }
    // Return the difference
    Some(ids_difference[0].clone())
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
    // Compute the squares -- PART 1
    let nb_occupied_squares = part_1(f_lines.clone());
    println!("Nb occupied squares is {}", nb_occupied_squares);
    // Check the ID -- PART 2
    let free_area = part_2(f_lines);
    match free_area {
        None => println!("Ooops, something goes wrong... It seems the free area to find is not alone (or does not exist) :/"),
        Some(id) => println!("The ID of the free area is {}", id),
    };
}
