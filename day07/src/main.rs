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

fn get_fuel_to_pos_2(crabs_pos: &Vec<i32>, dest: i32) -> i32 {
    let mut total_fuel = 0;

    for n in crabs_pos {
        let mut tmp: i32 = *n;
        let mut fuel_cost: i32 = 0;

        loop {
            if tmp < dest {
                tmp += 1;
                total_fuel += fuel_cost + 1;
                fuel_cost += 1;
            } else if tmp > dest {
                tmp -= 1;
                total_fuel += fuel_cost + 1;
                fuel_cost += 1;
            } else {
                break;
            }
        }
    }

    total_fuel
}

fn part2(fp: &str) {
    let crabs_pos: Vec<i32> = parse_file(fp);

    let mut min_fuel: i32 = i32::MAX;

    for i in 0..=2000 {
        let fuel_tmp = get_fuel_to_pos_2(&crabs_pos, i);
        if fuel_tmp < min_fuel {
            min_fuel = fuel_tmp;
        }
    }
    println!("Minimum fuel : {}", min_fuel);
}

fn get_fuel_to_pos(crabs_pos: &Vec<i32>, dest: i32) -> i32 {
    let mut total_fuel = 0;

    for n in crabs_pos {
        total_fuel += (n - dest).abs();
    }

    total_fuel
}

#[allow(dead_code)]
fn part1(fp: &str) {
    let crabs_pos: Vec<i32> = parse_file(fp);

    let mut min_fuel: i32 = i32::MAX;

    for i in 0..=2000 {
        let fuel_tmp = get_fuel_to_pos(&crabs_pos, i);
        if fuel_tmp < min_fuel {
            min_fuel = fuel_tmp;
        }
    }
    println!("Minimum fuel : {}", min_fuel);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day07 input.txt");
        std::process::exit(1);
    }
    // part1(&args[1]);
    part2(&args[1]);
}
