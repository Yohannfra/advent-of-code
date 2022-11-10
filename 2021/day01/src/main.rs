use std::env;
use std::fs;

#[allow(dead_code)]
fn part1() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        std::process::exit(1);
    }

    let fp = args[1].clone();

    let fc = fs::read_to_string(fp).expect("Could not read the file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let mut last = -1;
    let mut sum = 0;

    for l in lines {
        let num = match l.parse::<i32>() {
            Ok(n) => n,
            Err(_e) => continue,
        };
        if last != -1 && num > last {
            sum += 1;
        }
        last = num;
    }
    println!("{}", sum);
}

#[allow(dead_code)]
fn part2() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        std::process::exit(1);
    }

    let fc = fs::read_to_string(&args[1]).expect("Could not read the file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let mut nb_of_increase: i32 = -1;

    let mut previous_2 = lines[0].parse::<i32>().unwrap();
    let mut previous_1 = lines[1].parse::<i32>().unwrap();
    let mut previous_sum = 0;

    for i in 2..lines.len() - 1{
        let current: i32 = lines[i].parse::<i32>().unwrap();

        let sum = current + previous_1 + previous_2;
        if sum > previous_sum {
            nb_of_increase += 1;
        }
        previous_sum = sum;
        previous_2 = previous_1;
        previous_1 = current;
    }
    println!("{}", nb_of_increase);

}

fn main() {
    // part1();
    part2();
}
