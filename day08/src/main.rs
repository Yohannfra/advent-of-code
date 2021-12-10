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
                2|3|4|7 => sum_of_1478 += 1,
                _ => (),
            }
        }
    }
    println!("{}", sum_of_1478);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day08 input.txt");
        std::process::exit(1);
    }

    part1(Path::new(&args[1]));
    // part2(&args[1]);
}
