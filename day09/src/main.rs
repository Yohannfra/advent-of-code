use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

type U32vec2d = Vec<Vec<u32>>;

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

fn is_lowest_point(i: usize, j: usize, map: &U32vec2d) -> bool {
    if j > 0 && map[i][j] >= map[i][j - 1] {
        return false;
    }
    if i > 0 && map[i][j] >= map[i - 1][j] {
        return false;
    }
    if i != (*map).len() - 1 && map[i][j] >= map[i + 1][j] {
        return false;
    }
    if j != (*map[i]).len() - 1 && map[i][j] >= map[i][j + 1] {
        return false;
    }
    true
}

fn get_low_points(map: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut points: Vec<u32> = Vec::new();

    for i in 0usize..=(*map).len() - 1 {
        for j in 0usize..=map[i].len() - 1 {
            if ! is_lowest_point(i, j, map) {
                continue;
            }
            points.push(map[i][j]);
        }
    }
    points
}

fn expand_bassin(i: usize, j: usize, map: &mut U32vec2d) -> Vec<u32> {
    let mut bassin: Vec<u32> = Vec::new();

    if i > map.len() - 1 || j > map[i].len() -1 || map[i][j] == 9 {
        return bassin;
    }

    bassin.push(map[i][j]);
    map[i][j] = 9;

    if i > 0 {
        bassin.append(&mut expand_bassin(i-1, j, map));
    }
    if j > 0 {
        bassin.append(&mut expand_bassin(i, j-1, map));
    }
    bassin.append(&mut expand_bassin(i+1, j, map));
    bassin.append(&mut expand_bassin(i, j+1, map));

    bassin
}

fn get_multiple_of_sizes_of_three_largest_bassins(map: &mut U32vec2d) -> usize {
    let mut all_bassins: U32vec2d = Vec::new();

    for i in 0usize..=(*map).len() - 1 {
        for j in 0usize..=map[i].len() - 1 {
            if ! is_lowest_point(i, j, map) {
                continue;
            }
            let bassin: Vec<u32> = expand_bassin(i, j, map);
            all_bassins.push(bassin);
        }
    }
    let mut bassins_sizes: Vec<usize> = all_bassins.into_iter().map(|x| x.len()).collect();
    bassins_sizes.sort();
    bassins_sizes.reverse();

    bassins_sizes[0] * bassins_sizes[1] * bassins_sizes[2]
}

#[allow(dead_code)]
fn part2(fp: &Path) {
    let mut lines: U32vec2d = match parse_file(fp) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let res: usize = get_multiple_of_sizes_of_three_largest_bassins(&mut lines);
    println!("{}", res);
}

#[allow(dead_code)]
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
    // part1(Path::new(&args[1]));
    part2(Path::new(&args[1]));
}
