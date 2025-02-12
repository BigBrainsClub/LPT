use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

use crate::{config::Config, text::TextValidator};

pub fn start_threading(
    buffer: HashMap<String, Vec<Vec<u8>>>,
    config: &Config,
    threads: usize,
    filters: &[Vec<u8>],
) -> HashMap<String, Vec<TextValidator>> {
    let shared_config = config.clone();
    let shared_filters = filters.to_vec();
    let split_data = split_data(buffer, threads);
    let results: Vec<HashMap<String, Vec<TextValidator>>> = split_data
        .into_par_iter()
        .map(|chunk| {
            let mut results = HashMap::new();

            for (tag, lines) in chunk {
                let entry = results.entry(tag).or_insert_with(Vec::new);
                for line in lines {
                    if let Some(validator) = TextValidator::validate(&line, &shared_config, &shared_filters) {
                        entry.push(validator);
                    }
                }
            }
            results
        })
        .collect();
    results.into_iter().fold(HashMap::new(), |mut acc, res| {
        merge_maps(&mut acc, &res);
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
    target: &mut HashMap<String, Vec<TextValidator>>,
    source: &HashMap<String, Vec<TextValidator>>,
) {
    for (key, values) in source {
        target.entry(key.clone()).or_insert_with(Vec::new).extend(values.iter().cloned());
    }
}
