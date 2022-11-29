use std::{
    fs,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[allow(dead_code)]
pub fn read_file_to_str(_fp: impl AsRef<Path>) -> Option<String> {
    let fc = fs::read_to_string(_fp).expect("Could not read file");
    Some(fc)
}

#[allow(dead_code)]
pub fn read_file_to_vec(_fp: impl AsRef<Path>) -> Option<Vec<String>> {
    let file = File::open(_fp).expect("Could not read file");

    let buf = BufReader::new(file);
    Some(buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())
}

#[allow(dead_code)]
pub fn read_file_to_int_vec(_fp: impl AsRef<Path>) -> Option<Vec<i32>> {
    let file = File::open(_fp).expect("Could not read file");

    let buf = BufReader::new(file);
    Some(buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<i32>().unwrap())
        .collect())
}

#[allow(dead_code)]
pub fn read_file_to_float_vec(_fp: impl AsRef<Path>) -> Option<Vec<f32>> {
    let file = File::open(_fp).expect("Could not read file");

    let buf = BufReader::new(file);
    Some(buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<f32>().unwrap())
        .collect())
}
