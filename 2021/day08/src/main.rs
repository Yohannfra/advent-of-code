use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn parse_file(fp: &Path) -> Result<Vec<String>, io::Error> {
    let f = fs::File::open(fp)?;

    let reader = BufReader::new(f);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(String::from(line?));
    }

    Ok(lines)
}

#[allow(dead_code)]
fn part1(fp: &Path) {
    let lines: Vec<String> = match parse_file(fp) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let mut sum_of_1478 = 0;
    for l in lines {
        let part2: Vec<&str> = l.split("|").collect::<Vec<&str>>()[1]
            .trim()
            .split(" ")
            .collect();

        for w in part2 {
            match w.len() {
                2 | 3 | 4 | 7 => sum_of_1478 += 1,
                _ => (),
            }
        }
    }
    println!("{}", sum_of_1478);
}

fn count_nb_differences(s1: &str, s2: &str) -> u32 {
    let mut nb_diff = 0;

    for c in s1.chars() {
        match s2.find(c) {
            Some(_) => continue,
            _ => nb_diff += 1,
        }
    }
    nb_diff
}

fn get_digit_3(all_5: &Vec<&str>) -> String {
    // le 2 et 5 on deux différences -> isole le 3
    let digit_3: String = if count_nb_differences(all_5[0], all_5[1]) == 2 {
        all_5[2].to_string()
    } else if count_nb_differences(all_5[1], all_5[2]) == 2 {
        all_5[0].to_string()
    } else {
        all_5[1].to_string()
    };

    digit_3
}

fn translate_digits(p1: &Vec<&str>) -> Vec<String> {
    let mut digits: Vec<String> = vec!["".to_string(); 10];

    let mut all_len_five: Vec<&str> = Vec::new();
    let mut all_len_six: Vec<&str> = Vec::new();

    for signal in p1 {
        match signal.len() {
            2 => digits[1] = signal.to_string(),
            3 => digits[7] = signal.to_string(),
            4 => digits[4] = signal.to_string(),
            5 => all_len_five.push(&signal),
            6 => all_len_six.push(&signal),
            7 => digits[8] = signal.to_string(),
            _ => (),
        }
    }

    // get 3
    digits[3] = get_digit_3(&all_len_five);

    // remove 3 from vector all_five
    let all_len_five = all_len_five
        .into_iter()
        .filter(|&s| s != digits[3])
        .collect::<Vec<&str>>();

    // get 2 and 5
    if count_nb_differences(all_len_five[0], &digits[4]) == 3 {
        digits[2] = all_len_five[0].to_string();
        digits[5] = all_len_five[1].to_string();
    } else {
        digits[2] = all_len_five[1].to_string();
        digits[5] = all_len_five[0].to_string();
    }

    // get 0
    digits[0] = all_len_six
        .clone()
        .into_iter()
        .filter(|&s| count_nb_differences(s, &digits[5]) == 2)
        .collect::<Vec<&str>>()[0]
        .to_string();

    // remove 3 from vector all_five
    let all_len_six = all_len_six
        .into_iter()
        .filter(|&s| s != digits[0])
        .collect::<Vec<&str>>();

    // get 6 and 9
    if count_nb_differences(all_len_six[0], &digits[1]) == 4 {
        digits[9] = all_len_six[0].to_string();
        digits[6] = all_len_six[1].to_string();
    } else {
        digits[9] = all_len_six[1].to_string();
        digits[6] = all_len_six[0].to_string();
    }

    digits
}

fn sort_str(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

fn sum_digits(digits: &Vec<String>, on_screen: &Vec<&str>) -> i32 {
    let mut number_on_screen: String = String::new();

    for d in on_screen {
        let sorted_digit_on_screen: String = sort_str(d);
        let mut index_char: u8 = 0;
        for n in digits {
            if sorted_digit_on_screen == *n {
                number_on_screen.push((index_char + 48) as char);
            }
            index_char += 1;
        }
    }
    number_on_screen.parse::<i32>().unwrap()
}

fn part2(fp: &Path) {
    let lines: Vec<String> = match parse_file(fp) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let mut sum_of_all = 0;

    for l in lines {
        let part1: Vec<&str> = l.split("|").collect::<Vec<&str>>()[0]
            .trim()
            .split(" ")
            .collect();

        let part2: Vec<&str> = l.split("|").collect::<Vec<&str>>()[1]
            .trim()
            .split(" ")
            .collect();

        let digits: Vec<String> = translate_digits(&part1)
            .into_iter()
            .map(|s| sort_str(&s))
            .collect::<Vec<String>>();

        sum_of_all += sum_digits(&digits, &part2)
    }
    println!("{}", sum_of_all);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day08 input.txt");
        std::process::exit(1);
    }

    // part1(Path::new(&args[1]));
    part2(Path::new(&args[1]));
}
