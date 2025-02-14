use std::{fs::create_dir_all, path::PathBuf, sync::LazyLock};
use chrono::{Datelike, Local, Timelike};

pub mod config;
pub mod system;
pub mod file_io;
pub mod text;
pub mod logo;
pub mod reading;
pub mod threading;
pub mod counter;
pub mod writer;

pub const VERSION: &str = "1.1.2";
pub const TELEGRAM: &str = "@M0l0d0st_vnutri";
pub const FORUM: &str = "https://lolz.live/members/3060240";
pub const GITHUB: &str = "https://github.com/molodost-vnutri";
pub const THREAD: &str = "https://lolz.live/threads/5830632/";

pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap().join("./config.json"));
pub static ZAPROS_PATH: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap().join("./zapros.txt"));
pub static FILTER_PATH: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap().join("./filter.txt"));

pub const END_FILES_ALL_PARSE: &str = "full.txt";

pub const REPLACE_CHARS: &[u8] = b";| ";
pub const BAD_REPLACE_LIST: &[u8] = b"()*$!%&^#<>?~=[]+/\\,";
pub const RESULT_DIR: &str = "Results";

pub static CURRENT_DATE: LazyLock<String> = LazyLock::new(|| {
    let today = Local::now();
    format!("{}.{:02}.{:02} {:02}-{:02}-{:02}", today.year() % 100, today.month(), today.day(), today.hour(), today.minute(), today.second())
});


pub static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let path_result = PathBuf::from(RESULT_DIR).join(CURRENT_DATE.as_str());
    if !path_result.is_dir() {
        create_dir_all(&path_result).expect("Не удалось создать папку для результатов");
    }
    path_result
});