use std::collections::HashMap;
use std::time::Instant;
use memchr::memchr;
use memchr::memmem;

use indicatif::{ProgressBar, ProgressStyle};

use crate::counter::Counters;
use crate::system::clear_screen;
use crate::system::get_peak_memory_usage;
use crate::{
    config::Config, file_io::{BodySettings, LoaderBody, LoaderFiles},
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
    let mut counter = Counters::default();
    
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
        let mut buffer_process: Vec<Vec<u8>> = Vec::with_capacity(config.count_line_in_buffer as usize);
        for (chunk, len) in LoaderBody::new(file)? {
            let lines: Vec<Vec<u8>> = split_memchr(&chunk);
            counter.all_count += lines.len();
            pb_read.inc(len as u64);
            if buffer_process.len() < config.count_line_in_buffer as usize {
                buffer_process.extend(lines);
            } else {
                let sort = sorting_lines(&buffer_process, config.parse_zapros, &data.zapros);
                buffer_process.clear();
    
                let result = start_threading(sort, &config, threads, &data.filter);
                writer.write(&result, &config, &mut counter)?;
                pb_read.set_prefix(counter.format_multi_line(None, None, false));
            }

        }
        if !buffer_process.is_empty() {
            let sort = sorting_lines(&buffer_process, config.parse_zapros, &data.zapros);
            buffer_process.clear();

            let result = start_threading(sort, &config, threads, &data.filter);
            writer.write(&result, &config, &mut counter)?;
            pb_read.set_prefix(counter.format_multi_line(None, None, false));
        }
    }
    
    let duration = start.elapsed();
    clear_screen()?;
    if config.debug {
        let duration = format!("🚀 Время выполнения: {:?}", duration);
        let used_memory = format!("🧠 Пиковое потребление памяти: {} MB", get_peak_memory_usage() / (1024 * 1024));
        println!("{}", counter.format_multi_line(Some(duration), Some(used_memory), true))
    } else {
        println!("{}", counter.format_multi_line(None, None, false))
    }
    println!("Финишировал 🥇");
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
    zapros_vector: &[Vec<u8>]
) -> HashMap<String, Vec<Vec<u8>>> {
    if !zapros {
        return HashMap::from([("default".to_string(), lines.to_vec())]);
    }

    let mut result = HashMap::new();

    for zapros in zapros_vector {
        let zapros_str = match std::str::from_utf8(zapros) {
            Ok(s) => s.to_lowercase().to_string(),
            Err(_) => continue,
        };
        let finder = memmem::Finder::new(zapros);

        let matched_lines: Vec<Vec<u8>> = lines.iter()
            .filter(|line| finder.find(&line.to_ascii_lowercase()).is_some())
            .cloned()
            .collect();

        if !matched_lines.is_empty() {
            result.insert(zapros_str, matched_lines);
        }
    }

    result
}