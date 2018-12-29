#[macro_use]
extern crate lazy_static;
extern crate rayon;
extern crate regex;

use std::cmp;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::{self, FromStr};

use rayon::prelude::*;

struct Points {
    points: Vec<Point>,
}

struct Drawing {
    minx: isize,
    miny: isize,
    maxx: isize,
    maxy: isize,
}

#[derive(Debug)]
struct Point {
    current_x: isize,
    current_y: isize,
    velocity_x: isize,
    velocity_y: isize,
}

trait Normalize {
    type I;
    type O;

    fn normalize(i: Self::I, m: Self::I) -> Self::O;
}

impl Normalize for isize {
    type I = isize;
    type O = usize;

    fn normalize(i: Self::I, m: Self::I) -> Self::O {
        if i >= 0 {
            (i - m) as Self::O
        } else {
            (i + m.abs()) as Self::O
        }
    }
}

impl Drawing {
    fn dimensions(&self) -> (usize, usize) {
        (
            (self.maxx - self.minx + 1).abs() as usize,
            (self.maxy - self.miny + 1).abs() as usize,
        )
    }
}

impl Points {
    fn new(points: Vec<Point>) -> Points {
        Points { points: points }
    }

    fn next(&mut self) {
        self.points.par_iter_mut().for_each(|p| p.next());
    }

    fn get_draw(&self) -> Drawing {
        let mut d = Drawing {
            minx: self.points[0].current_x,
            miny: self.points[0].current_y,
            maxx: self.points[0].current_x,
            maxy: self.points[0].current_y,
        };
        for p in &self.points {
            d.minx = cmp::min(d.minx, p.current_x);
            d.miny = cmp::min(d.miny, p.current_y);
            d.maxx = cmp::max(d.maxx, p.current_x);
            d.maxy = cmp::max(d.maxy, p.current_y);
        }
        d
    }

    fn draw(&mut self) -> String {
        let d = self.get_draw();
        let dim = d.dimensions();
        let mut grid_inlined = String::new();
        if dim.0 > 100 || dim.1 > 100 {
            return grid_inlined;
        }
        let mut grid = vec![vec![b'.'; dim.0]; dim.1];
        for p in &self.points {
            let x = isize::normalize(p.current_x, d.minx);
            let y = isize::normalize(p.current_y, d.miny);
            grid[y][x] = b'#';
        }
        for row in grid {
            grid_inlined.push_str(str::from_utf8(&row).unwrap());
            grid_inlined.push('\n');
        }
        grid_inlined
    }
}

impl Point {
    fn next(&mut self) {
        self.current_x = self.current_x + self.velocity_x;
        self.current_y = self.current_y + self.velocity_y;
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(
                r"position=<\s*(?P<cx>-?[0-9]+),\s*(?P<cy>-?[0-9]+)>\s+velocity=<\s*(?P<vx>-?[0-9]+),\s*(?P<vy>-?[0-9]+)>"
            )
            .unwrap();
        }

        let caps = match RE.captures(s) {
            None => panic!("Error capturing the stream"),
            Some(caps) => caps,
        };

        Ok(Point {
            current_x: caps["cx"].parse()?,
            current_y: caps["cy"].parse()?,
            velocity_x: caps["vx"].parse()?,
            velocity_y: caps["vy"].parse()?,
        })
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
    let mut points: Points = Points::new(points);
    for time in 0..1_000_000 {
        let draw = points.draw();
        if draw.len() == 0 {
            continue;
        }
        println!("Second: {}", time);
        println!("{}", draw);
        println!("********************************");
        points.next();
    }
}
