use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn parse_file(fp: &Path) -> Result<Vec<Vec<u32>>, io::Error> {
    let f = fs::File::open(fp)?;
    let reader = BufReader::new(f);

    let mut lines: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let v: Vec<u32> = l.chars().map(|x| x.to_digit(10).unwrap()).collect();
                lines.push(v)
            }
            _ => continue,
        }
    }
    Ok(lines)
}

fn get_low_points(map: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut points: Vec<u32> = Vec::new();

    for i in 0usize..=(*map).len() - 1 {
        for j in 0usize..=map[i].len() - 1 {
            if j > 0 && map[i][j] >= map[i][j - 1] {
                continue;
            }
            if i > 0 && map[i][j] >= map[i - 1][j] {
                continue;
            }
            if i != (*map).len() - 1 && map[i][j] >= map[i + 1][j] {
                continue;
            }
            if j != (*map[i]).len() - 1 && map[i][j] >= map[i][j + 1] {
                continue;
            }
            points.push(map[i][j]);
        }
    }
    points
}

fn part1(fp: &Path) {
    let lines: Vec<Vec<u32>> = match parse_file(fp) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let low_points: Vec<u32> = get_low_points(&lines);
    let risk_level: Vec<u32> = low_points.into_iter().map(|x| x + 1).collect();
    println!("{}", risk_level.iter().sum::<u32>());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day09 input.txt");
        std::process::exit(1);
    }
    part1(Path::new(&args[1]));
    // part2(Path::new(&args[1]));
}
