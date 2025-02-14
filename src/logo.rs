use std::sync::LazyLock;
use colored::Colorize;

use crate::{VERSION, TELEGRAM, THREAD, FORUM, GITHUB};

pub static LOGO_PRINT: LazyLock<String> = LazyLock::new(|| {
    format!(
      "  ____  _         ____            _              _____ _       _     
 |  _ \\(_)       |  _ \\          (_)            / ____| |     | |    
 | |_) |_  __ _  | |_) |_ __ __ _ _ _ __  ___  | |    | |_   _| |__  
 |  _ <| |/ _` | |  _ <| '__/ _` | | '_ \\/ __| | |    | | | | | '_ \\ 
 | |_) | | (_| | | |_) | | | (_| | | | | \\__ \\ | |____| | |_| | |_) |
 |____/|_|\\__, | |____/|_|  \\__,_|_|_| |_|___/  \\_____|_|\\__,_|_.__/ 
           __/ |
          |___/")
    .bright_red()
    .to_string()
});

pub static INFORMATION: LazyLock<String> = LazyLock::new(|| {
    get_logo("", "", "", "", "", "", "", "", "", "", "")
});

pub fn get_logo(a: &str, b: &str, c: &str, d: &str, e: &str, f: &str, t: &str, g: &str, i: &str, u: &str, y: &str) -> String {
    format!(
        "\n╔═══════════════════════════════════════════════════════════╗{}\n\
           ║ Сделал molodost vnutri для форума lolzteam                ║{}\n\
           ║ Контакты и ссылки для фидбека:                            ║{}\n\
           ║                                                           ║{}\n\
           ║ [{}] => {}                            ║{}\n\
           ║ [{}] => {}  ║{}\n\
           ║ [{}] => {}    ║{}\n\
           ║ [{}] => {}            ║{}\n\
           ║                                                           ║{}\n\
           ║ {}: {}                                             ║{}\n\
           ╚═══════════════════════════════════════════════════════════╝{}\n",
           a,
           b,
           d,
           c,
          "telegram".bright_blue(), TELEGRAM, e,
          "Ссылка на профиль".green(), FORUM, f,
          "Тема на форуме".green(), THREAD, t,
          "Github".bright_black(), GITHUB, g, i,
          "Версия".bright_green(), VERSION, u, y
      )
}

pub static BIG_DICKS_TO: LazyLock<String> = LazyLock::new(|| {
    format!("
    ┌──────────────────┐
    │and big dick to{} │
    └──────────────────┘", " √".green()).bright_black().to_string()
});

pub fn logo(info: &str) -> String {
    format!("{}{}{}", *LOGO_PRINT, *BIG_DICKS_TO, info)
}
