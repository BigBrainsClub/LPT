use std::{
    fs::File, 
    io::{stdin, Write, Read}, 
};
use serde::{Deserialize, Serialize};
use crate::CONFIG_PATH;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub parse_zapros: bool,
    pub parse_email: bool,
    pub parse_login: bool,
    pub parse_number: bool,
    pub parse_full: bool,
    pub find_data_type: bool,
    pub check_length: bool,
    pub default_length: (u8, u8),
    pub all_length: (u8, u8),
    pub email_length: (u8, u8),
    pub login_length: (u8, u8),
    pub number_length: (u8, u8),
    pub password_length: (u8, u8),
    pub remove_equal_login_password: bool,
    pub login_to_lower_case: bool,
    pub autothreads: bool,
    pub threads: u8,
    pub count_line_in_buffer: u32,
    pub debug: bool
}

impl AsRef<Config> for Config {
    fn as_ref(&self) -> &Config {
        self
    }
}

impl Config {
    pub fn get_string_config(&self) -> std::io::Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| e.into())
    }

    pub fn load_config() -> std::io::Result<Self> {
        let path = CONFIG_PATH.to_path_buf();
        if path.is_file() {
            let mut file = File::open(&path)?;
            let mut json_data = String::new();
            file.read_to_string(&mut json_data)?;
            return serde_json::from_str(&json_data).map_err(|e| e.into());
        }
        let default_config = Self::default();
        let mut config_file = File::create(&path)?;
        config_file.write_all(default_config.get_string_config()?.as_bytes())?;

        println!("Конфигурация создана. Настройте файл {} и нажмите Enter...", path.display());
        stdin().read_line(&mut String::new())?;
        Self::load_config()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            parse_zapros: false,
            parse_email: true,
            parse_login: true,
            parse_number: true,
            parse_full: false,
            find_data_type: true,
            check_length: true,
            default_length: (5, 35),
            all_length: (20, 150),
            email_length: (8, 35),
            login_length: (5, 35),
            number_length: (11, 16),
            password_length: (8, 35),
            remove_equal_login_password: true,
            login_to_lower_case: true,
            autothreads: true,
            threads: 1,
            count_line_in_buffer: 5000,
            debug: false
        }
    }
}