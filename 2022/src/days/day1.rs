use crate::utils;

pub fn part1(fp: &str) -> Option<String> {
    let fc = utils::read_file_to_vec(fp).unwrap();

    let mut max_food = 0;

    let mut current_sum = 0;

    for l in fc {
        if l.is_empty() {
            if current_sum > max_food {
                max_food = current_sum;
            }
            current_sum = 0;
            continue;
        }
        current_sum += l.parse::<i32>().unwrap();
    }

    Some(max_food.to_string())
}

pub fn part2(fp: &str) -> Option<String> {
    let fc = utils::read_file_to_vec(fp).unwrap();
    let mut all_sums: Vec<i32> = Vec::new();

    let mut current_sum = 0;

    for l in fc {
        if l.is_empty() {
            all_sums.push(current_sum);
            current_sum = 0;
            continue;
        }
        current_sum += l.parse::<i32>().unwrap();
    }
    all_sums.push(current_sum);

    all_sums.sort();
    all_sums.reverse();

    Some((all_sums[0] + all_sums[1] + all_sums[2]).to_string())
}
