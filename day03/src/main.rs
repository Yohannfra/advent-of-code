use std::env;
use std::fs;

#[allow(dead_code)]
fn part1() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: ./day03 fp");
        std::process::exit(1);
    }

    let fp = args[1].clone();
    let fc = fs::read_to_string(fp).expect("Could not read file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let mut bits_frequencies: Vec<i32> = vec![0; 12];

    let line_len = lines[0].len() - 1;

    for idx in 0..=line_len {
        for l in &lines {
            if l.len() == 0 {
                continue;
            }
            if l.as_bytes()[idx] as char == '1' {
                bits_frequencies[idx] += 1;
            } else {
                bits_frequencies[idx] -= 1;
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..=line_len {
        if bits_frequencies[i] > 0 {
            gamma_rate |= 1 << line_len-i
        } else {
            epsilon_rate |= 1 << line_len-i
        }
    }
    println!("gamma_rate: {} x epsilon_rate: {} = power consumption: {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}

fn part2() {

}

fn main() {
    // part1();
    part2();
}
