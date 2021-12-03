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
            gamma_rate |= 1 << line_len - i
        } else {
            epsilon_rate |= 1 << line_len - i
        }
    }
    println!(
        "gamma_rate: {} x epsilon_rate: {} = power consumption: {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );
}

// part 2

#[allow(dead_code)]
fn get_bits_frequency(lines: &Vec<&str>) -> Vec<i32> {
    let mut bits_frequencies: Vec<i32> = vec![0; 12];

    let line_len = lines[0].len() - 1;

    for idx in 0..=line_len {
        for l in lines {
            if l.is_empty() {
                continue;
            }
            if l.as_bytes()[idx] as char == '1' {
                bits_frequencies[idx] += 1;
            } else {
                bits_frequencies[idx] -= 1;
            }
        }
    }

    return bits_frequencies;
}

#[allow(dead_code)]
fn get_co2_or_oxygen_rating(lines: Vec<&str>, comparators: &str, index: usize) -> isize {
    if lines.len() == 1 {
        return isize::from_str_radix(lines[0], 2).unwrap()
    }

    let bits_frequencies: Vec<i32> = get_bits_frequency(&lines);

    let mut lines_tmp: Vec<&str> = Vec::new();

    for l in &lines {
        if l.is_empty() {
            continue;
        }
        let c: char = l.as_bytes()[index] as char;

        if bits_frequencies[index] > 0 && c == comparators.as_bytes()[0] as char {
            lines_tmp.push(l);
        } else if bits_frequencies[index] < 0 && c == comparators.as_bytes()[1] as char {
            lines_tmp.push(l);
        } else if bits_frequencies[index] == 0 && c == comparators.as_bytes()[2] as char {
            lines_tmp.push(l);
        }
    }
    return get_co2_or_oxygen_rating(lines_tmp, comparators, index + 1);
}

#[allow(dead_code)]
fn part2() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: ./day03 fp");
        std::process::exit(1);
    }

    let fp = args[1].clone();
    let fc = fs::read_to_string(fp).expect("Could not read file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let oxygen_generator_rating = get_co2_or_oxygen_rating(lines.clone(), "101", 0);
    let co2_scrubber_rating = get_co2_or_oxygen_rating(lines.clone(), "010", 0);

    let res = oxygen_generator_rating * co2_scrubber_rating;
    println!("life support rating = {}", res);
}

fn main() {
    // part1();
    part2();
}
