use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

const OPEN_CHARS: &str = "([{<";
const CLOSE_CHARS: &str = ")]}>";

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

    let mut sum_of_errors: u64 = 0;

    let mut chars_vec: Vec<char> = Vec::new();

    for l in lines {
        for c in l.chars() {
            if OPEN_CHARS.contains(c) {
                chars_vec.push(c);
            } else {
                let last_char_in: char = *chars_vec.last().unwrap();
                if OPEN_CHARS.find(last_char_in) == CLOSE_CHARS.find(c) {
                    chars_vec.pop();
                    continue;
                }
                sum_of_errors += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                };
                break;
            }
        }
        chars_vec.clear();
    }
    println!("{}", sum_of_errors);
}

fn is_valid_line(l: &str) -> bool {
    let mut chars_vec: Vec<char> = Vec::new();

    for c in l.chars() {
        if OPEN_CHARS.contains(c) {
            chars_vec.push(c);
        } else {
            let last_char_in: char = *chars_vec.last().unwrap();
            if OPEN_CHARS.find(last_char_in) != CLOSE_CHARS.find(c) {
                return false;
            }
            chars_vec.pop();
        }
    }
    true
}

fn complete_line(l: &str) -> String {
    let mut eol: String = String::new();

    let mut chars_vec: Vec<char> = Vec::new();

    for c in l.chars() {
        if OPEN_CHARS.contains(c) {
            chars_vec.push(c);
        } else {
            let last_char_in: char = *chars_vec.last().unwrap();
            if OPEN_CHARS.find(last_char_in) != CLOSE_CHARS.find(c) {
                continue;
            }
            chars_vec.pop();
        }
    }

    while !chars_vec.is_empty() {
        let c: char = *chars_vec.last().unwrap();
        chars_vec.pop();
        match c {
            '[' => eol.push(']'),
            '<' => eol.push('>'),
            '{' => eol.push('}'),
            '(' => eol.push(')'),
            _ => panic!("Invalid char"),
        }
    }
    eol
}

fn count_completion_score(s: &str) -> u64 {
    let mut n: u64 = 0;

    for c in s.chars() {
        n *= 5;
        n += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Invalid char"),
        }
    }
    n
}

#[allow(dead_code)]
fn part2(fp: &Path) {
    let lines: Vec<String> = match parse_file(fp) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let mut all_scores: Vec<u64> = Vec::new();

    for l in lines {
        if !is_valid_line(&l) {
            continue;
        }
        let end_of_line: String = complete_line(&l);
        all_scores.push(count_completion_score(&end_of_line));
    }
    all_scores.sort();
    println!("{:?}", all_scores[all_scores.len() / 2]);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day10 input.txt");
        std::process::exit(1);
    }

    // part1(Path::new(&args[1]));
    part2(Path::new(&args[1]));
}
