use colored::Colorize;

use crate::system::clear_screen;

pub struct Logo {
    version: &'static str,
    telegram: &'static str,
    forum: &'static str,
    github: &'static str,
    thread: &'static str
}

impl Logo {
    pub fn logo() {
        clear_screen();
        let logo = Logo {
            version: "1.0.0",
            telegram: "@M0l0d0st_vnutri",
            forum: "https://zelenka.guru/members/3060240",
            github: "https://github.com/M0l0d0st",
            thread: "https://lolz.live/threads/5830632/"
        };
        let body = "
██╗░░░░░░█████╗░██╗░░░░░███████╗████████╗███████╗░█████╗░███╗░░░███╗
██║░░░░░██╔══██╗██║░░░░░╚════██║╚══██╔══╝██╔════╝██╔══██╗████╗░████║
██║░░░░░██║░░██║██║░░░░░░░███╔═╝░░░██║░░░█████╗░░███████║██╔████╔██║
██║░░░░░██║░░██║██║░░░░░██╔══╝░░░░░██║░░░██╔══╝░░██╔══██║██║╚██╔╝██║
███████╗╚█████╔╝███████╗███████╗░░░██║░░░███████╗██║░░██║██║░╚═╝░██║
╚══════╝░╚════╝░╚══════╝╚══════╝░░░╚═╝░░░╚══════╝╚═╝░░╚═╝╚═╝░░░░░╚═╝
        ".green();
        let info = format!("        Сделал molodost vnutri для форума zelenka.guru
        Контакты и ссылки для фидбека:
             [{}]=> {}
             [{}]=> {}
             [{}]=> {}
             [{}]=> {}
        {}: {}\n\n",
        "telegram".bright_blue(),
        logo.telegram,
        "Ссылка на профиль".green(),
        logo.forum,
        "Тема на форуме".green(),
        logo.thread,
        "Github".bright_black(),
        logo.github,
        "Версия".bright_green(),
        logo.version
    );
    print!("{}\n{}", body, info);
    }
}