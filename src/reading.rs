use std::{collections::HashMap, fs::OpenOptions, io::{BufRead, BufReader}, path::{Path, PathBuf}};

use regex::Regex;

use crate::{config::Config, schemes::ThreadResult, writer};




pub fn reading_file(path: &Path, config: &Config, zapros: &Vec<String>, filter_vector: &Vec<String>, threads: usize, regex_tuple: (Regex, Regex, Regex), result_path: &PathBuf) -> ThreadResult {
    let mut result = ThreadResult::new();
    match OpenOptions::new().read(true).open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut buffer: HashMap<String, Vec<String>> = HashMap::new();
            for line in reader.lines().filter_map(|x|x.ok()) {
                push_to_buffer(&mut buffer, config, zapros, &line);
                if !check_owerbuffer(&buffer, config) {
                    append_result(&mut result, result_path, &buffer, config, regex_tuple.clone(), filter_vector, threads);
                    buffer = HashMap::new();
                }
            } if !buffer.is_empty() {
                append_result(&mut result, result_path, &buffer, config, regex_tuple.clone(), filter_vector, threads);
            }
        }
        Err(e) => println!("Ошибка при чтении файла: {}", e)
    }
    result
}

fn push_to_buffer(buffer: &mut HashMap<String, Vec<String>>, config: &Config, zapros: &Vec<String>, line: &str) {
    if config.parse_zapros {
        for zapros in zapros {
            if line.to_lowercase().contains(zapros) {
                buffer.entry(zapros.to_string()).or_insert(Vec::new()).push(line.to_string());
            }
        }
    } else {
        buffer.entry("default_data".to_owned()).or_insert(Vec::new()).push(line.to_owned());
    }
}

fn check_owerbuffer(buffer: &HashMap<String, Vec<String>>, config: &Config) -> bool {
    let mut buffer_count: usize = 0;
    for (_, value) in buffer {
        buffer_count += value.len();
    }
    config.count_line_in_buffer as usize >= buffer_count
}

fn append_result(result: &mut ThreadResult, result_path: &PathBuf, buffer: &HashMap<String, Vec<String>>, config: &Config, regex_tuple: (Regex, Regex, Regex), filter_vector: &Vec<String>, threads: usize) {
    let result_threads = ThreadResult::start_threads(&buffer, config, regex_tuple, filter_vector, threads);
    result.bad_word += result_threads.bad_word;
    result.length_all += result_threads.length_all;
    result.length_credit += result_threads.length_credit;
    result.not_ulp += result_threads.not_ulp;
    result.regex_error += result_threads.regex_error;
    result.total_count += writer::write_result(result_path, &result_threads.result, config.parse_full);
}