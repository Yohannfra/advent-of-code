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

    let mut sum_of_errors: u64 = 0;

    let mut chars_vec: Vec<char> = Vec::new();

    const OPEN_CHARS: &str = "([{<";
    const CLOSE_CHARS: &str = ")]}>";

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

#[allow(dead_code)]
fn part2(fp: &Path) {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day08 input.txt");
        std::process::exit(1);
    }

    part1(Path::new(&args[1]));
    // part2(Path::new(&args[1]));
}
