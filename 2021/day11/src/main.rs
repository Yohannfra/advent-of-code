use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

type U32vec2d = Vec<Vec<u32>>;

fn parse_file(fp: &Path) -> Result<Vec<Vec<u32>>, io::Error> {
    let f = fs::File::open(fp)?;
    let reader = BufReader::new(f);

    let mut lines: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let v: Vec<u32> = l.chars().map(|x| x.to_digit(10).unwrap()).collect();
                lines.push(v)
            }
            _ => continue,
        }
    }
    Ok(lines)
}

fn pretty_print_arr(arr: &U32vec2d) {
    println!("");
    for l in arr {
        for n in l {
            print!("{}", n);
        }
        println!("");
    }
    println!("");
}

fn flash(lines: &mut U32vec2d, i: usize, j: usize) {
    if lines[i][j] == 10 {
        return;
    }

    lines[i][j] = 10;

    const ALL_AROUND: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for pos in ALL_AROUND {
        let x: i32 = (i as i8 + pos.0) as i32;
        let y: i32 = (j as i8 + pos.1) as i32;

        if x < 0 || y < 0 || x as usize > lines.len() - 1 || y as usize > lines[i].len() - 1 {
            continue;
        }

        lines[x as usize][y as usize] += 1;
        if lines[x as usize][y as usize] == 9 {
            flash(lines, x as usize, y as usize);
        }
    }
}

fn make_flashes(lines: &mut U32vec2d) {
    for i in 0..=lines.len() - 1 {
        for j in 0..=lines[i].len() - 1 {
            if lines[i][j] == 9 {
                flash(lines, i, j);
            }
        }
    }
}

#[allow(dead_code)]
fn part1(fp: &Path) {
    let mut lines: U32vec2d = match parse_file(fp) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    const NB_OF_STEPS: i32 = 2;

    println!("Initial state:");
    pretty_print_arr(&lines);

    for _ in 0..=NB_OF_STEPS {
        // increment all numbers by 1
        lines = lines
            .into_iter()
            .map(|l| {
                l.into_iter()
                    .map(|x| if x < 9 { x + 1 } else { x })
                    .collect()
            })
            .collect();

        // check if any need a flash and flash it
        make_flashes(&mut lines);

        pretty_print_arr(&lines);
    }
}

#[allow(dead_code)]
fn part2(fp: &Path) {
    // TODO
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: day11 input.txt");
        std::process::exit(1);
    }

    part1(Path::new(&args[1]));
    // part2(Path::new(&args[1]));
}
