use std::env;
use std::fs;

fn parse_file(fp: &str) -> Vec<i32> {
    let fc: String = fs::read_to_string(fp).expect("Could not read the file");
    let line: Vec<&str> = (fc.split("\n").collect::<Vec<&str>>())[0].split(",").collect();

    let mut fishes: Vec<i32> = Vec::new();

    for i in line {
        fishes.push(i.parse::<i32>().unwrap());
    }
    fishes
}

fn print_fishes_state(start: String, fishes: &Vec<i32>) {
    print!("{}", start);

    for i in 0..=fishes.len() - 1 {
        print!("{}", fishes[i]);
        if i != fishes.len() - 1 {
            print!(",");
        }
    }
    print!("\n");
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

fn part1(fp: &str) {
    let mut fishes: Vec<i32> = parse_file(fp);

    // print_fishes_state("Initial State:\t\t".to_string(), &fishes);

    for days in 1..=80 {
        reproduce_fishes(&mut fishes);
        // print_fishes_state(format!("After\t{} {}\t\t", days, if days == 1 { "day" } else { "days" }), &fishes);
    }

    println!("Total = {}", fishes.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day05 input.txt");
        std::process::exit(1);
    }
    part1(&args[1]);
}
