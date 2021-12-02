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

    let fp = args[1].clone();

    let fc = fs::read_to_string(fp).expect("Could not read the file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let mut last1 = -1;
    let mut last2 = -1;
    let mut last_sum = 0;
    let mut nb_increments = 0;

    for l in lines {
        let num = match l.parse::<i32>() {
            Ok(n) => n,
            Err(_e) => continue,
        };

        let sum = last1 + last2 + num;
        if last_sum > 0 && last1 != -1 && last2 != -1 && sum > last_sum {
            let c: char = if sum > last_sum {
                '+'
            } else {
                '-'
            };
            println!(
                "{} + {} + {} = {} > {}   {}",
                last2,
                last1,
                num,
                sum,
                last_sum,
                c
            );
            // println!("{} {} {}", last2, last1, num);
            nb_increments+= 1;
            // print!(" : Increased {} > {}", last1 + last2 + num, last_sum);
        }

        if last1 != -1 {
            last2 = last1;
            last1 = num;
            last_sum = last1 + last2 + num;
        } else {
            last1 = num;
        }
    }
    println!("{}", nb_increments);
}

fn main() {
    // part1();
    part2();
}
