use std::sync::LazyLock;
use colored::Colorize;

use crate::{VERSION, TELEGRAM, THREAD, FORUM, GITHUB};

static LOGO_PRINT: LazyLock<String> = LazyLock::new(|| {
    concat!(
      "\n██╗░░░░░░█████╗░██╗░░░░░███████╗████████╗███████╗░█████╗░███╗░░░███╗\n",
        "██║░░░░░██╔══██╗██║░░░░░╚════██║╚══██╔══╝██╔════╝██╔══██╗████╗░████║\n",
        "██║░░░░░██║░░██║██║░░░░░░░███╔═╝░░░██║░░░█████╗░░███████║██╔████╔██║\n",
        "██║░░░░░██║░░██║██║░░░░░██╔══╝░░░░░██║░░░██╔══╝░░██╔══██║██║╚██╔╝██║\n",
        "███████╗╚█████╔╝███████╗███████╗░░░██║░░░███████╗██║░░██║██║░╚═╝░██║\n",
        "╚══════╝░╚════╝░╚══════╝╚══════╝░░░╚═╝░░░╚══════╝╚═╝░░╚═╝╚═╝░░░░░╚═╝\n"
    )
    .green()
    .to_string()
});

static INFORMATION: LazyLock<String> = LazyLock::new(|| {
    format!(
        "Сделал molodost vnutri для форума lolz.live\n\
         Контакты и ссылки для фидбека:\n\
         \n\
         \t[{}] => {}\n\
         \t[{}] => {}\n\
         \t[{}] => {}\n\
         \t[{}] => {}\n\
         \n\
         {}: {}\n",
        "telegram".bright_blue(), TELEGRAM,
        "Ссылка на профиль".green(), FORUM,
        "Тема на форуме".green(), THREAD,
        "Github".bright_black(), GITHUB,
        "Версия".bright_green(), VERSION
    )
});

pub fn print_logo() {
    println!("{}\n{}", *LOGO_PRINT, *INFORMATION);
}
