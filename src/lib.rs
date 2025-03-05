use std::{
    fs::create_dir_all,
    path::PathBuf,
    sync::LazyLock,
};

use big_brains_club_logo::LogoBuilder;
use chrono::{Datelike, Local, Timelike};
use colored::Colorize;

mod config;
mod file_io;
mod reading;
mod system;
mod threading;
mod writer;

pub use reading::init;

const VERSION: &str = "1.1.5";
const TELEGRAM: &str = "@M0l0d0st_vnutri";
const FORUM: &str = "https://lolz.live/members/3060240";
const GITHUB: &str = "https://github.com/BigBrainsClub/";
const GITHUB_MAIL: &str = "90547216+BigBrainsClub@users.noreply.github.com";
const THREAD: &str = "https://lolz.live/threads/5830632/";
const HEADER_LOGO: &str = "LPT (Login Password from Text)";

const INFO_STR: &str = "═════════════════════INFO════════════════════";
const ALL_COUNT_STR: &str = "📊 Всего строк";
const ERROR_LENGTH_STR: &str = "❌ Ошибка валидации длины";
const ERROR_LP_STR: &str = "🔑 Ошибка валидации lp";
const ERROR_PARSE_STR: &str = "⚠️ Ошибка парса строки";
const ERROR_FILTER_STR: &str = "🚫 Ошибка валидации фильтрами";
const ERROR_LP_EQ_STR: &str = "🔁 Ошибка одинаковые log_pass";
const VALID_COUNT_STR: &str = "✅ Валидных строк";
const DEBUG_STR: &str = "══════════════════DEBUG MODE═════════════════";
const DURATION_STR: &str = "🚀 Время выполнения: ";
const PEEK_MEMORY_STR: &str = "🧠 Пиковое потребление памяти:";
const FINISH_STR: &str = "Финишировал 🥇";
const END_FILES_ALL_PARSE: &str = "full.txt";

const REPLACE_CHARS: &[u8] = b";| ";
const BAD_REPLACE_LIST: &[u8] = b"()*$!%&^#<>?~=[]+/\\,";
const RESULT_DIR: &str = "Results";

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| 
    std::env::current_dir().unwrap().join("config.json")
);

static ZAPROS_PATH: LazyLock<PathBuf> = LazyLock::new(|| 
    std::env::current_dir().unwrap().join("zapros.txt")
);

static FILTER_PATH: LazyLock<PathBuf> = LazyLock::new(|| 
    std::env::current_dir().unwrap().join("filter.txt")
);

static CURRENT_DATE: LazyLock<String> = LazyLock::new(|| {
    let today = Local::now();
    format!(
        "{:02}.{:02}.{:02} {:02}-{:02}-{:02}",
        today.year() % 100,
        today.month(),
        today.day(),
        today.hour(),
        today.minute(),
        today.second()
    )
});

static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = PathBuf::from(RESULT_DIR).join(&*CURRENT_DATE);
    create_dir_all(&path).expect("Failed to create results directory");
    path
});

static LOGO: LazyLock<LogoBuilder> = LazyLock::new(|| {
    let mut builder = LogoBuilder::default();
    builder
        .entry_info(vec![
            ("Ссылки для фидбека".bold().to_string().leak(), None),
            ("Telegram".bright_blue().to_string().leak(), Some(TELEGRAM)),
            ("Форум".green().to_string().leak(), Some(FORUM)),
            ("Тема".green().to_string().leak(), Some(THREAD)),
            ("GitHub".bright_black().to_string().leak(), Some(GITHUB)),
            ("GitHub Mail".bright_black().to_string().leak(), Some(GITHUB_MAIL)),
            ("Версия".bright_green().to_string().leak(), Some(VERSION)),
        ])
        .with_custom_header(HEADER_LOGO);
    builder
});

static RENDERED_LOGO: LazyLock<String> = LazyLock::new(|| {
    LOGO.render()
});