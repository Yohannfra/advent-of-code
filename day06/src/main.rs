use std::env;
use std::fs;

fn parse_file(fp: &str) -> Vec<i32> {
    let fc: String = fs::read_to_string(fp).expect("Could not read the file");
    let line: Vec<&str> = (fc.split("\n").collect::<Vec<&str>>())[0]
        .split(",")
        .collect();

    let mut fishes: Vec<i32> = Vec::new();

    for i in line {
        fishes.push(i.parse::<i32>().unwrap());
    }
    fishes
}

fn reproduce_fishes(fishes: &mut Vec<i32>) {
    let mut new_fishes: Vec<i32> = Vec::new();

    for i in 0..fishes.len() {
        if fishes[i] > 0 {
            fishes[i] -= 1;
        } else {
            fishes[i] = 6;
            new_fishes.push(8);
        }
    }
    fishes.append(&mut new_fishes);
}

fn reproduce_fishes_2(fishes: &mut Vec<i64>) {
    for i in 1..=9 {
        let tmp: i64 = fishes[i];
        fishes[i] = 0;
        fishes[i - 1] += tmp;
    }
    let tmp: i64 = fishes[0];
    fishes[0] = 0;
    fishes[7] += tmp;
    fishes[9] += tmp;
}

fn part2(fp: &str) {
    let all_fishes: Vec<i32> = parse_file(fp);

    let mut fishes: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for f in all_fishes {
        fishes[f as usize + 1] += 1
    }

    for days in 1..=256 {
        reproduce_fishes_2(&mut fishes);
    }

    let sum: i64 = fishes.iter().sum();
    println!("Total = {}", sum);
}

fn part1(fp: &str) {
    let mut fishes: Vec<i32> = parse_file(fp);

    for days in 1..=80 {
        reproduce_fishes(&mut fishes);
    }
    println!("Total = {}", fishes.len());
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
