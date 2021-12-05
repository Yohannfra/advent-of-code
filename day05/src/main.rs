use std::env;
use std::fs;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line {
    p1: Point,
    p2: Point,
    path: Vec<Point>,
}

fn parse_lines(fp: &str) -> Vec<Line> {
    let fc = fs::read_to_string(fp).expect("Could not read the file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let mut all_lines: Vec<Line> = Vec::new();

    for l in lines {
        if l.is_empty() {
            continue;
        }
        let tmp_line: String = l.clone().replace(" ", "").replace("->", ",");
        let spl: Vec<&str> = tmp_line.split(",").collect();
        let new_line: Line = Line {
            p1: Point {
                x: spl[0].parse::<i32>().unwrap(),
                y: spl[1].parse::<i32>().unwrap(),
            },
            p2: Point {
                x: spl[2].parse::<i32>().unwrap(),
                y: spl[3].parse::<i32>().unwrap(),
            },
            path: Vec::new(),
        };
        all_lines.push(new_line);
    }
    all_lines
}

fn part1() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day05 input.txt");
        std::process::exit(1);
    }

    let mut all_lines = parse_lines(&args[1]);

    for l in &mut all_lines {
        if l.p1.x != l.p2.x && l.p1.y != l.p2.y {
            continue
        }
        if l.p1.x == l.p2.x {
            for i in if l.p1.y < l.p2.y{ l.p1.y..=l.p2.y } else { l.p2.y..=l.p1.y } {
                (*l).path.push(Point {x: l.p1.x, y: i});
            }
        } else {
            for i in if l.p1.x < l.p2.x { l.p1.x..=l.p2.x} else { l.p2.x..=l.p1.x } {
                (*l).path.push(Point {x: i, y: l.p1.y});
            }
        }
    }
    let mut map: HashMap<Point, i32>  = HashMap::new();

    for l in all_lines {
        for p in l.path {
            if ! map.contains_key(&p) {
                map.insert(p, 1);
            } else {
                *map.get_mut(&p).unwrap() += 1;
            }
        }
    }

    let mut sum_of_points = 0;

    for p in map.keys() {
        if map[&p] >= 2 {
            sum_of_points += 1;
        }
    }

    println!("{}", sum_of_points);
}

fn main() {
    part1();
}
