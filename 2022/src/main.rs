use std::env;

mod utils;
mod days;

const CURRENT_DAY: i32 = 1; // 1-25
const PART_OF_DAY: i32 = 1; // 1-2

fn main() {
    let args: Vec<String> = env::args().collect();

    let res = match CURRENT_DAY {
        1 => if PART_OF_DAY == 1 { days::day1::part1(&args[1]) } else {days::day1::part2(&args[1])},
        2 => if PART_OF_DAY == 1 {  days::day2::part1(&args[1]) } else { days::day2::part2(&args[1])},
        3 => if PART_OF_DAY == 1 {  days::day3::part1(&args[1]) } else { days::day3::part2(&args[1])},
        4 => if PART_OF_DAY == 1 {  days::day4::part1(&args[1]) } else { days::day4::part2(&args[1])},
        5 => if PART_OF_DAY == 1 {  days::day5::part1(&args[1]) } else { days::day5::part2(&args[1])},
        6 => if PART_OF_DAY == 1 {  days::day6::part1(&args[1]) } else { days::day6::part2(&args[1])},
        7 => if PART_OF_DAY == 1 {  days::day7::part1(&args[1]) } else { days::day7::part2(&args[1])},
        8 => if PART_OF_DAY == 1 {  days::day8::part1(&args[1]) } else { days::day8::part2(&args[1])},
        9 => if PART_OF_DAY == 1 {  days::day9::part1(&args[1]) } else { days::day9::part2(&args[1])},
        10 => if PART_OF_DAY == 1 { days::day10::part1(&args[1]) } else {days::day10::part2(&args[1])},
        11 => if PART_OF_DAY == 1 { days::day11::part1(&args[1]) } else {days::day11::part2(&args[1])},
        12 => if PART_OF_DAY == 1 { days::day12::part1(&args[1]) } else {days::day12::part2(&args[1])},
        13 => if PART_OF_DAY == 1 { days::day13::part1(&args[1]) } else {days::day13::part2(&args[1])},
        14 => if PART_OF_DAY == 1 { days::day14::part1(&args[1]) } else {days::day14::part2(&args[1])},
        15 => if PART_OF_DAY == 1 { days::day15::part1(&args[1]) } else {days::day15::part2(&args[1])},
        16 => if PART_OF_DAY == 1 { days::day16::part1(&args[1]) } else {days::day16::part2(&args[1])},
        17 => if PART_OF_DAY == 1 { days::day17::part1(&args[1]) } else {days::day17::part2(&args[1])},
        18 => if PART_OF_DAY == 1 { days::day18::part1(&args[1]) } else {days::day18::part2(&args[1])},
        19 => if PART_OF_DAY == 1 { days::day19::part1(&args[1]) } else {days::day19::part2(&args[1])},
        20 => if PART_OF_DAY == 1 { days::day20::part1(&args[1]) } else {days::day20::part2(&args[1])},
        21 => if PART_OF_DAY == 1 { days::day21::part1(&args[1]) } else {days::day21::part2(&args[1])},
        22 => if PART_OF_DAY == 1 { days::day22::part1(&args[1]) } else {days::day22::part2(&args[1])},
        23 => if PART_OF_DAY == 1 { days::day23::part1(&args[1]) } else {days::day23::part2(&args[1])},
        24 => if PART_OF_DAY == 1 { days::day24::part1(&args[1]) } else {days::day24::part2(&args[1])},
        25 => if PART_OF_DAY == 1 { days::day25::part1(&args[1]) } else {days::day25::part2(&args[1])},
        _ => panic!("Invalid day"),
    };

    println!("{:?}", res);
}
