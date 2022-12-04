use crate::utils;
use regex::Regex;

pub fn part1(fp: &str) -> Option<String> {
    let fc = utils::read_file_to_vec(fp).unwrap();
    let mut nb_pairs = 0;

    for l in fc {
        let r = Regex::new(r"^([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)$").unwrap();
        let caps = r.captures(&l)?;

        let min1 = caps.get(1)?.as_str().to_string().parse::<i32>().unwrap();
        let max1 = caps.get(2)?.as_str().to_string().parse::<i32>().unwrap();
        let min2 = caps.get(3)?.as_str().to_string().parse::<i32>().unwrap();
        let max2 = caps.get(4)?.as_str().to_string().parse::<i32>().unwrap();

        if (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)  {
            nb_pairs+=1;
        }
    }
    Some(nb_pairs.to_string())
}

pub fn part2(_fp: &str) -> Option<String> {
    Some("".to_owned())
}
