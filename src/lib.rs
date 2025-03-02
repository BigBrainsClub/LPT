use std::{fs::create_dir_all, path::PathBuf, sync::LazyLock};
use big_brains_club_logo::LogoBuilder;
use chrono::{Datelike, Local, Timelike};
use colored::Colorize;

mod config;
mod system;
mod file_io;
mod reading;
mod threading;
mod counter;
mod writer;

pub use reading::init;

const VERSION: &str = "1.1.4";
const TELEGRAM: &str = "@M0l0d0st_vnutri";
const FORUM: &str = "https://lolz.live/members/3060240";
const GITHUB: &str = "https://github.com/BigBrainsClub/";
const GITHUB_MAIN: &str = "90547216+BigBrainsClub@users.noreply.github.com";
const THREAD: &str = "https://lolz.live/threads/5830632/";

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap().join("./config.json"));
static ZAPROS_PATH: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap().join("./zapros.txt"));
static FILTER_PATH: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap().join("./filter.txt"));

const END_FILES_ALL_PARSE: &str = "full.txt";

const REPLACE_CHARS: &[u8] = b";| ";
const BAD_REPLACE_LIST: &[u8] = b"()*$!%&^#<>?~=[]+/\\,";
const RESULT_DIR: &str = "Results";

static CURRENT_DATE: LazyLock<String> = LazyLock::new(|| {
    let today = Local::now();
    format!("{}.{:02}.{:02} {:02}-{:02}-{:02}", today.year() % 100, today.month(), today.day(), today.hour(), today.minute(), today.second())
});


static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let path_result = PathBuf::from(RESULT_DIR).join(CURRENT_DATE.as_str());
    if !path_result.is_dir() {
        create_dir_all(&path_result).expect("Не удалось создать папку для результатов");
    }
    path_result
});

static LOGO: LazyLock<LogoBuilder> = LazyLock::new(|| {
    LogoBuilder::new(
        vec![
            ("Ссылки для фидбека".bold().to_string().leak(), None),
            ("telegram".bright_blue().to_string().leak(), Some(TELEGRAM)),
            ("Ссылка на профиль".green().to_string().leak(), Some(FORUM)),
            ("Тема на форуме".green().to_string().leak(), Some(THREAD)),
            ("Github".bright_black().to_string().leak(), Some(GITHUB)),
            ("Почта Github".bright_black().to_string().leak(), Some(GITHUB_MAIN)),
            ("Версия".bright_green().to_string().leak(), Some(VERSION))
        ],
        vec![],
        "=>"
    ).with_custom_header("LPT (Login Password from Text)")
});

static LOGO_READY: LazyLock<String> = LazyLock::new(|| {
    (LOGO).clone().render()
});