use crate::utils;

pub fn part1(fp: &str) -> Option<String> {
    let fc = utils::read_file_to_vec(fp).unwrap();
    let mut num_2 = 0;
    let mut num_3 = 0;


    for l in fc {
        let mut has_num2 = false;
        let mut has_num3 = false;
        for c in l.chars() {
            if l.matches(c).count() == 2 && !has_num2 {
                num_2 += 1;
                has_num2 = true;
            }
            if l.matches(c).count() == 3 && !has_num3 {
                num_3 += 1;
                has_num3 = true;
            }
        }
    }
    Some((num_3 * num_2).to_string())
}

pub fn part2(_fp: &str) -> Option<String> {
    Some("".to_owned())
}
