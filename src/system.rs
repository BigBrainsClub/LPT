use std::process::Command;

use chrono::{Datelike, Local};
use num_cpus::get;

use crate::config::Config;

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

pub fn get_threads(config: &Config) -> usize {
    if config.autothreads {
        return get() - 1
    }
    return config.threads as usize
}

pub fn get_current_data_path() -> String {
    let today = Local::now();

    let year = today.year() % 100;
    let month = today.month();
    let day = today.day();

    format!("{:02}-{:02}-{:02}", year, month, day)
}