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

fn print_sum_of_overlap(all_lines: Vec<Line>) {
    let mut map: HashMap<Point, i32> = HashMap::new();

    for l in all_lines {
        for p in l.path {
            if !map.contains_key(&p) {
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

#[allow(dead_code)]
fn part1(fp: &str) {
    let mut all_lines: Vec<Line> = parse_lines(fp);

    for l in &mut all_lines {
        if l.p1.x != l.p2.x && l.p1.y != l.p2.y {
            continue;
        }
        if l.p1.x == l.p2.x {
            for i in if l.p1.y < l.p2.y { l.p1.y..=l.p2.y } else { l.p2.y..=l.p1.y } {
                (*l).path.push(Point { x: l.p1.x, y: i });
            }
        } else {
            for i in if l.p1.x < l.p2.x { l.p1.x..=l.p2.x } else { l.p2.x..=l.p1.x } {
                (*l).path.push(Point { x: i, y: l.p1.y });
            }
        }
    }

    print_sum_of_overlap(all_lines);
}

fn is_a_45_degrees_diagonal(l: &Line) -> bool {
    (l.p1.x - l.p2.x).abs() == (l.p1.y - l.p2.y).abs()
}

fn get_diagonal_path(l: &Line) -> Vec<Point> {
    let mut path: Vec<Point> = Vec::new();

    let mut p1: Point = Point {
        x: l.p1.x,
        y: l.p1.y,
    };
    let p2: Point = Point {
        x: l.p2.x,
        y: l.p2.y,
    };

    path.push(Point { x: p1.x, y: p1.y });

    loop {
        p1.x += if p1.x < p2.x { 1 } else { -1 };
        p1.y += if p1.y < p2.y { 1 } else { -1 };

        path.push(Point { x: p1.x, y: p1.y });
        if p1 == p2 {
            break;
        }
    }
    path
}

#[allow(dead_code)]
fn part2(fp: &str) {
    let mut all_lines: Vec<Line> = parse_lines(fp);

    for l in &mut all_lines {
        if l.p1.x != l.p2.x && l.p1.y != l.p2.y {
            if is_a_45_degrees_diagonal(&l) {
                (*l).path.append(&mut get_diagonal_path(&l));
            }
            continue;
        }
        if l.p1.x == l.p2.x {
            for i in if l.p1.y < l.p2.y { l.p1.y..=l.p2.y } else { l.p2.y..=l.p1.y } {
                (*l).path.push(Point { x: l.p1.x, y: i });
            }
        } else {
            for i in if l.p1.x < l.p2.x { l.p1.x..=l.p2.x } else { l.p2.x..=l.p1.x } {
                (*l).path.push(Point { x: i, y: l.p1.y });
            }
        }
    }
    print_sum_of_overlap(all_lines);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day05 input.txt");
        std::process::exit(1);
    }

    // part1(&args[1]);
    part2(&args[1]);
}
