use std::collections::{HashMap, HashSet};
use rayon::prelude::*;
use smallvec::SmallVec;
use vulp::{LocalConfig, ValidationError, VULP, ResultVULP};

use crate::{config::Config, BAD_REPLACE_LIST, REPLACE_CHARS};

pub fn start_threading<'a> (
    buffer: HashMap<String, Vec<Vec<u8>>>,
    config: &Config,
    threads: usize,
    filters: &SmallVec<[SmallVec<[u8; 16]>; 4]>,
) -> HashMap<String, Vec<Result<ResultVULP, ValidationError>>> {
    let shared_config = LocalConfig::new(
        REPLACE_CHARS,
        BAD_REPLACE_LIST,
        filters.clone(),
        config.all_length,
        config.email_length,
        config.login_length,
        config.number_length,
        config.default_length,
        config.password_length,
        config.login_to_lower_case,
        config.parse_email,
        config.parse_login,
        config.parse_number,
        config.find_data_type,
        config.remove_equal_login_password,
        config.check_length
    );
    let split_data = split_data(buffer, threads);
    let results: Vec<HashMap<String, Vec<Result<ResultVULP, ValidationError>>>> = split_data
        .into_par_iter()
        .map(|chunk| {
            let mut validator = VULP::new(&shared_config);
            let mut results = HashMap::new();

            for (tag, lines) in &chunk {
                let entry = results.entry(tag.to_string()).or_insert_with(Vec::new);
                for line in lines {
                    let result = validator.validate(&line);
                    entry.push(result);
                }
            }
            results
        })
        .collect();
    results.into_iter().fold(HashMap::new(), |mut acc: HashMap<String, Vec<Result<ResultVULP, ValidationError>>>, res: HashMap<String, Vec<Result<ResultVULP, ValidationError>>>| {
        merge_maps(&mut acc, res);
        acc
    })
}
fn split_data(
    input: HashMap<String, Vec<Vec<u8>>>,
    threads: usize,
) -> Vec<HashMap<String, Vec<Vec<u8>>>> {
    let mut result = vec![HashMap::new(); threads];

    let mut used_strings = HashSet::new();

    for (key, values) in input {
        let unique_values: Vec<Vec<u8>> = values.into_iter().filter(|v| used_strings.insert(v.clone())).collect();

        let total_values = unique_values.len();
        if total_values == 0 || threads == 0 {
            continue;
        }

        let chunk_size = (total_values + threads - 1) / threads;

        for (i, chunk) in unique_values.chunks(chunk_size).enumerate() {
            result[i].entry(key.clone()).or_insert_with(Vec::new).extend_from_slice(chunk);
        }
    }

    result
}
fn merge_maps(
    target: &mut HashMap<String, Vec<Result<ResultVULP, ValidationError>>>,
    source: HashMap<String, Vec<Result<ResultVULP, ValidationError>>>,
) {
    for (key, values) in source {
        target.entry(key.clone()).or_insert_with(Vec::new).extend(values);
    }
}
