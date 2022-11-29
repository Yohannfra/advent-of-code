use crate::utils;

pub fn part1(fp: &str) -> Option<String> {
    let fc = utils::read_file_to_vec(fp).unwrap();

    let mut freq = 0;

    for l in fc {
        freq += match l.chars().nth(0).unwrap() {
            '+' => l.clone()[1..].parse::<i32>().unwrap(),
            '-' => - l.clone()[1..].parse::<i32>().unwrap(),
            _ => panic!(),
        }
    }
    Some(freq.to_string())
}

pub fn part2(_fp: &str) -> Option<String> {
    Some("".to_owned())
}
