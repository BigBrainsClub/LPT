use std::collections::HashMap;
use std::time::Instant;

use aho_corasick::AhoCorasick;
use memchr::memchr;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use reader_vlf::Reader;
use smallvec::SmallVec;

use crate::system::clear_screen;
use crate::system::get_peak_memory_usage;
use crate::ALL_COUNT_STR;
use crate::DEBUG_STR;
use crate::DURATION_STR;
use crate::ERROR_FILTER_STR;
use crate::ERROR_LENGTH_STR;
use crate::ERROR_LP_EQ_STR;
use crate::ERROR_LP_STR;
use crate::ERROR_PARSE_STR;
use crate::FINISH_STR;
use crate::INFO_STR;
use crate::LOGO;
use crate::PEEK_MEMORY_STR;
use crate::VALID_COUNT_STR;
use crate::{
    config::Config, file_io::{BodySettings, LoaderFiles},
    system::get_threads, threading::start_threading, writer::Writer
};

pub fn init() -> std::io::Result<()> {
    let config = Config::load_config()?;
    BodySettings::load_init()?;
    let path = LoaderFiles::get_path()?;
    let data = BodySettings::new()?;
    let files = LoaderFiles::new(&path)?;
    let threads = get_threads(&config);
    let mut writer = Writer::new();
    let mut logo_counter = LOGO.clone();
    logo_counter
        .entry_extra(INFO_STR, None)
        .entry_extra(ALL_COUNT_STR, Some(0))
        .entry_extra(ERROR_LENGTH_STR, Some(0))
        .entry_extra(ERROR_LP_STR, Some(0))
        .entry_extra(ERROR_PARSE_STR, Some(0))
        .entry_extra(ERROR_FILTER_STR, Some(0))
        .entry_extra(ERROR_LP_EQ_STR, Some(0))
        .entry_extra(VALID_COUNT_STR, Some(0));
    
    let start = Instant::now();
    for file in files {
        if let Err(_) = LoaderFiles::init_file(&path) {
            continue;
        }
        let pb_read = ProgressBar::new(LoaderFiles::init_file(&path)?);
        pb_read.set_style(
            ProgressStyle::with_template(&format!(
                "{}{}{}",
                "{prefix}\n{spinner:.red} [Обработка файла ",
                file.file_name().unwrap().to_string_lossy(),
                "]{wide_bar:.green}] {bytes}/{total_bytes} ({eta})"
            ))
            .unwrap()
            .progress_chars("#>-"),
        );
        pb_read.set_prefix(logo_counter.render());
        let mut buffer_process: Vec<Vec<u8>> = Vec::with_capacity(config.count_line_in_buffer as usize);
        for (chunk, len) in Reader::new(file)? {
            let lines: Vec<Vec<u8>> = split_memchr(&chunk);
            logo_counter.entry_extra(ALL_COUNT_STR, Some(lines.len()));
            pb_read.inc(len as u64);
            if buffer_process.len() < config.count_line_in_buffer as usize {
                buffer_process.extend(lines);
            } else {
                let sort = sorting_lines(&buffer_process, config.parse_zapros, &data.zapros);
                buffer_process.clear();
    
                let result = start_threading(sort, &config, threads, &data.filter);
                writer.write(&result, &config, &mut logo_counter)?;
                pb_read.set_prefix(logo_counter.render());
            }

        }
        if !buffer_process.is_empty() {
            let sort = sorting_lines(&buffer_process, config.parse_zapros, &data.zapros);
            buffer_process.clear();

            let result = start_threading(sort, &config, threads, &data.filter);
            writer.write(&result, &config, &mut logo_counter)?;
            pb_read.set_prefix(logo_counter.render());
        }
    }
    
    let duration = start.elapsed();
    clear_screen()?;
    if config.debug {
        logo_counter
            .entry_extra(DEBUG_STR, None)
            .entry_extra(&format!("{} {:?}", DURATION_STR, duration), None)
            .entry_extra(&format!("{} {} MB", PEEK_MEMORY_STR, get_peak_memory_usage() / (1024 * 1024)), None);
        println!("{}", logo_counter.render());
    } else {
        println!("{}", logo_counter.render())
    }
    println!("{}", FINISH_STR);
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}

fn split_memchr(input: &[u8]) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let mut start = 0;

    while let Some(pos) = memchr(b'\n', &input[start..]) {
        let end = start + pos;
        let line = &input[start..end];
        let line = if line.ends_with(b"\r") { &line[..line.len() - 1] } else { line };
        result.push(line.to_vec());
        start = end + 1;
    }

    if start < input.len() {
        let last_line = if input[start..].ends_with(b"\r") {
            &input[start..input.len() - 1]
        } else {
            &input[start..]
        };
        result.push(last_line.to_vec());
    }

    result
}

fn sorting_lines(
    lines: &[Vec<u8>],
    zapros: bool,
    zapros_vector: &SmallVec<[SmallVec<[u8; 16]>; 4]>,
) -> HashMap<String, Vec<Vec<u8>>> {
    if !zapros {
        return HashMap::from([("default".to_string(), lines.to_vec())]);
    }

    let patterns: Vec<String> = zapros_vector
        .iter()
        .filter_map(|v| std::str::from_utf8(v).ok().map(|s| s.to_lowercase()))
        .collect();

    let ac = AhoCorasick::new(&patterns).unwrap();

    lines
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut acc, line| {
                let lower_line = line.to_ascii_lowercase();
                for mat in ac.find_iter(&lower_line) {
                    let pattern = &patterns[mat.pattern()];
                    acc.entry(pattern.clone()).or_insert_with(Vec::new).push(line.clone());
                }
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut acc, map| {
                for (key, value) in map {
                    acc.entry(key).or_insert_with(Vec::new).extend(value);
                }
                acc
            },
        )
}