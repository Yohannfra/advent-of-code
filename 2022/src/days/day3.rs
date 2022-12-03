use crate::utils;

pub fn part1(fp: &str) -> Option<String> {
    let fc = utils::read_file_to_vec(fp).unwrap();

    let mut sum:i32 = 0;

    for l in &fc {
        let part1 = l[0..l.len()/2].to_owned();
        let part2 = l[l.len()/2..].to_owned();

        for c in part1.chars() {
            if part2.chars().into_iter().any(|x|x==c) {
                sum += c as i32 - if c.is_lowercase() {96} else {64-26};
                break;
            }
        }
    }

    Some(sum.to_string())
}

pub fn part2(fp: &str) -> Option<String> {
    let fc = utils::read_file_to_vec(fp).unwrap();

    let mut sum:i32 = 0;

    for i in (0..fc.len()-2).step_by(3) {
        let l1 = fc[i].clone();
        let l2 = fc[i+1].clone();
        let l3 = fc[i+2].clone();

        for c in l1.chars() {
            if l2.chars().into_iter().any(|x|x==c) && l3.chars().into_iter().any(|x|x==c){
                sum += c as i32 - if c.is_lowercase() {96} else {64-26};
                break;
            }
        }

    }

    Some(sum.to_string())
}
