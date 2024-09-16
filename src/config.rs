use std::{io::{stdin, Write}, path::Path};

use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Config {
    pub parse_zapros: bool,
    pub parse_email: bool,
    pub parse_login: bool,
    pub parse_number: bool,
    pub parse_full: bool,

    pub use_regex: bool,
    pub check_length: bool,

    pub default_length: (u8, u8),
    pub all_length: (u8, u8),

    pub email_length: (u8, u8),
    pub login_length: (u8, u8),
    pub number_length: (u8, u8),
    pub password_length: (u8, u8),

    pub autothreads: bool,
    pub threads: u8,
    pub count_line_in_buffer: u32
}

impl Config {
    pub fn new() -> Self {
        Config {
            parse_zapros: false,
            parse_email: true,
            parse_login: true,
            parse_number: true,
            parse_full: false,

            use_regex: true,
            check_length: true,

            default_length: (5, 35),
            all_length: (20, 150),
            email_length: (8, 35),
            login_length: (5, 35),
            number_length: (11, 16),
            password_length: (8, 35),

            autothreads: true,
            threads: 4,
            count_line_in_buffer: 500_000
        }
    }

    pub fn get_string_config(self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn load_config(self, config_path: &Path) -> Self {
        if config_path.exists() {
            match std::fs::read_to_string(config_path) {
                Ok(body) => {
                    match serde_json::from_str(&body) {
                        Ok(config) => return config,
                        Err(e) => panic!("{}", e),
                    }
                },
                Err(e) => panic!("{}", e),
            }
        }
        else {
            let config_file = std::fs::File::create(config_path);
            match config_file {
                Ok(mut file) => {
                    match file.write_all(self.get_string_config().as_bytes()) {
                        Ok(_) => {
                            println!("Конфигурация была создана, настройте конфиг и нажмите enter");
                            stdin().read_line(&mut String::new()).unwrap();
                            return self.load_config(config_path);
                        },
                        Err(e) => panic!("{}", e),
                    }
                }
                Err(e) => panic!("{}", e), 
            }
        }
    }
}