use std::env;
use std::fs;

#[allow(dead_code)]
fn part1() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: ./day02 fp");
        std::process::exit(1);
    }

    let fp = args[1].clone();
    let fc = fs::read_to_string(fp).expect("Could not read file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let mut initial_pos_x = 0;
    let mut initial_pos_z = 0;


    for l in lines {
        if l.len() == 0 {
            continue;
        }

        let splitted_line: Vec<&str> = l.split(" ").collect();
        let command = splitted_line[0];
        let nb = splitted_line[1].parse::<i32>().unwrap();

        match command {
            "up" => {initial_pos_z -= nb},
            "down" => {initial_pos_z += nb},
            "forward" => {initial_pos_x += nb},
            &_ => continue,
        }
    }
    println!("x: {} z: {}", initial_pos_x, initial_pos_z);
    println!("{} * {} = {}", initial_pos_x, initial_pos_z, initial_pos_z * initial_pos_x);
}

#[allow(dead_code)]
fn part2() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: ./day02 fp");
        std::process::exit(1);
    }

    let fp = args[1].clone();
    let fc = fs::read_to_string(fp).expect("Could not read file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let mut pos_x = 0;
    let mut pos_z = 0;
    let mut aim = 0;


    for l in lines {
        if l.len() == 0 {
            continue;
        }

        let splitted_line: Vec<&str> = l.split(" ").collect();
        let command = splitted_line[0];
        let nb = splitted_line[1].parse::<i32>().unwrap();

        match command {
            "up" => {
                aim -= nb;
            },
            "down" => {
                aim += nb;
            },
            "forward" => {
                pos_x += nb;
                pos_z += aim * nb;
            },
            &_ => continue,
        }
    }
    println!("x: {} z: {}", pos_x, pos_z);
    println!("{} * {} = {}", pos_x, pos_z, pos_z * pos_x);

}

fn main() {
    // part1();
    part2();
}
