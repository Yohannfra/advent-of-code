use std::env;
use std::fs;

fn parse_grids(lines: Vec<&str>) -> Vec<Vec<Vec<i32>>> {
    let mut all_grids: Vec<Vec<Vec<i32>>> = Vec::new();

    let mut grid_tmp = vec![vec![0; 5]; 5];
    let mut index_in_grid_tmp: usize = 0;
    for l in lines {
        if l.is_empty() || l.len() > 15 {
            // skip empty lines and first line
            if l.is_empty() && grid_tmp[0][0] != 0 {
                all_grids.push(grid_tmp.clone());
                index_in_grid_tmp = 0;
            }
            continue;
        }

        let line_tmp: String = String::from(l).replace("  ", " ").trim().to_string();
        let splitted_line: Vec<&str> = line_tmp.split(" ").collect();
        for i in 0..=4 {
            grid_tmp[index_in_grid_tmp][i] = splitted_line[i].parse::<i32>().unwrap();
        }
        index_in_grid_tmp += 1;
    }
    all_grids
}

fn check_grids_win(grids: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut idx: i32 = 0;

    for grid in grids {
        // rows
        for row in 0usize..=4 {
            if grid[row] == [-1, -1, -1, -1, -1] {
                return idx;
            }
        }

        // cols
        // for col in 0usize..=4 {
        for index in 0usize..=4 {
            let tmp: Vec<i32> = vec![
                grid[0][index],
                grid[1][index],
                grid[2][index],
                grid[3][index],
                grid[4][index],
            ];
            if tmp == [-1, -1, -1, -1, -1] {
                return idx;
            }
        }
        // }

        idx += 1;
    }
    -1
}

fn mark_grid(grids: &mut Vec<Vec<Vec<i32>>>, number: i32) {
    for grid in grids {
        for row in 0usize..=4 {
            for i in 0usize..=4 {
                if grid[row][i] == number {
                    grid[row][i] = -1;
                }
            }
        }
    }
}

fn sum_unmarkeds_numbers(grid: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for rows in grid {
        for n in rows {
            if *n != -1 {
                sum += n;
            }
        }
    }
    sum
}

#[allow(dead_code)]
fn part1() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: ./day04 input.txt");
        std::process::exit(1);
    }

    let fp = args[1].clone();
    let fc = fs::read_to_string(fp).expect("Could not read file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let random_numbers: Vec<&str> = lines[0].split(",").collect();

    let mut all_grids = parse_grids(lines);

    for n in random_numbers {
        let number = n.parse::<i32>().unwrap();
        mark_grid(&mut all_grids, number);

        let winner: i32 = check_grids_win(&all_grids);
        if winner != -1 {
            println!(
                "Winner score is : {}",
                sum_unmarkeds_numbers(&all_grids[winner as usize]) * number
            );
            std::process::exit(0);
        }
    }
}

fn part2() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: ./day04 input.txt");
        std::process::exit(1);
    }

    let fp = args[1].clone();
    let fc = fs::read_to_string(fp).expect("Could not read file");
    let lines: Vec<&str> = fc.split("\n").collect();

    let random_numbers: Vec<&str> = lines[0].split(",").collect();

    let mut all_grids = parse_grids(lines);

    let mut win_number: i32 = 0;
    let mut last_win_grid: Vec<Vec<i32>> = Vec::new();

    for n in random_numbers {
        let number = n.parse::<i32>().unwrap();
        mark_grid(&mut all_grids, number);

        loop {
            let winner: i32 = check_grids_win(&all_grids);
            if winner != -1 {
                win_number = number;
                last_win_grid = all_grids[winner as usize].clone();
                all_grids.remove(winner as usize);
            } else {
                break;
            }
        }
    }
    println!("Last winner score is : {}", sum_unmarkeds_numbers(&last_win_grid) * win_number);
}

fn main() {
    // part1();
    part2();
}
