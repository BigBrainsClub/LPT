use std::{fs::OpenOptions, io::{BufRead, BufReader}};

const FILTER_PATH: &str = "filter.txt";
const ZAPROS_PATH: &str = "zapros.txt";


pub fn load_configuration() -> (Vec<String>, Vec<String>) {
    let mut filter_vector = Vec::new();
    let mut zapros_vector = Vec::new();
    let filter = OpenOptions::new().read(true).open(FILTER_PATH).unwrap();
    let zapros = OpenOptions::new().read(true).open(ZAPROS_PATH).unwrap();
    let reader = BufReader::new(filter);
    for line in reader.lines().filter_map(|x|x.ok()) {
        filter_vector.push(line);
    }
    let reader = BufReader::new(zapros);
    for line in reader.lines().filter_map(|x|x.ok()) {
        zapros_vector.push(line);
    }

    (zapros_vector, filter_vector)
}