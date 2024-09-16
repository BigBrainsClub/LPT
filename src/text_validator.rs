use regex::Regex;
use url::Url;

use crate::{config::Config, schemes::{DataEnum, LineEnum, TextResult, TextValidator, UnzipLineParts}};

const REPLACE_LIST: [char; 3] = [';', ' ', '|'];
const BAD_REPLACE_LIST: [char; 21] = ['(', ')', '*', '$', '!', '%', '&', '^', '#', '<', '>', '?', ';', '~', '=', '[', ']', '+', '/', '\\', ','];

impl TextValidator {
    fn get_type_line(&self, line: &str) -> LineEnum {
        if line.starts_with("http://")
        || line.starts_with("https://") {
            return LineEnum::Http
        }
        else if line.starts_with("android://") {
            return LineEnum::Android
        }
        else if line.matches("http").count() == 1
        && line.matches("://").count() == 1 {
            return LineEnum::ReversedHttp
        }
        LineEnum::WithoutHttp
    }


    fn validate_host(&self, url: &str) -> Option<(String, Option<u16>)> {
        let url = Url::parse(url);
        match url {
            Ok(url) => {
                match url.host_str() {
                    Some(host) => {
                        return Some((host.to_owned(), url.port()))
                    },
                    None => {}
                }
            },
            Err(_) => {}
        }
        None
    }


    fn get_parts_in_line(&self, line: &str) -> Option<UnzipLineParts> {
        let mut line = line.to_owned();
        for char in REPLACE_LIST {
            if line.contains(char) {
                line = line.replace(char, ":");
            }
        }

        if !(2..=5).contains(&line.matches(":").count()) {
            return None;
        }

        let mut parts = line.split(":").collect::<Vec<&str>>();

        let mut result = UnzipLineParts {
            url: String::new(),
            port: None,
            login: String::new(),
            password: String::new(),
            linetype: LineEnum::Default
        };

        result.linetype = self.get_type_line(&line);


        if result.linetype == LineEnum::ReversedHttp {
            parts.reverse();
            result.login = parts.pop().unwrap().to_owned();
            result.password = parts.pop().unwrap().to_owned();
            parts.reverse();
            result.url = parts.join(":");
            result.linetype = LineEnum::Http;
        }
        else {
            result.password = parts.pop().unwrap().to_owned();
            result.login = parts.pop().unwrap().to_owned();
            result.url = parts.join(":");
            if result.linetype == LineEnum::WithoutHttp {
                result.url = format!("http://{}", result.url);
                result.linetype = LineEnum::Http;
            }
        }
        for char in BAD_REPLACE_LIST {
            if result.login.contains(char) {
                result.login = result.login.replace(char, "");
            }
        }
        match result.linetype {
            LineEnum::Android => return Some(result),
            LineEnum::Http => {
                match self.validate_host(&result.url) {
                    Some((host, port)) => {
                        result.url = host;
                        result.port = port;
                        return Some(result)
                    },
                    None => {}
                }
            },
            _ => {}
        }
        None
    }


    fn validate_full_length(&self, line: &str, config: &Config) -> bool {
        let line_length = line.len();
        (config.all_length.0 as usize..=config.all_length.1 as usize)
        .contains(&line_length)
    }


    fn checking_bad_words_in_credits(&self, filter_vector: &Vec<String>) -> bool {
        let credits = format!("{}:{}", self.login.to_lowercase(), self.password.to_lowercase());
        filter_vector.iter().any(|x| credits.contains(x))
    }

    fn find_type_credits(&self, config: &Config, regex_tuple: &(&Regex, &Regex, &Regex)) -> DataEnum {
        if !config.use_regex {
            return DataEnum::NonUsed
        }

        if config.parse_email {
            if regex_tuple.0.is_match(&self.login) {
                return DataEnum::Email
            }
        }
        if config.parse_login {
            if regex_tuple.1.is_match(&self.login) {
                return DataEnum::Login
            }
        }
        if config.parse_number {
            if regex_tuple.2.is_match(&self.login) {
                return DataEnum::Number
            }
        }
        DataEnum::NonValid
    }

    fn validate_credit_length(&self, config: &Config) -> bool {
        if !config.check_length {
            return true
        }
        if !(config.password_length.0 as usize..=config.password_length.1 as usize).contains(&self.password.len()) {
            return false
        }
        if !config.use_regex {
            return (config.default_length.0 as usize..=config.default_length.1 as usize).contains(&self.login.len())
        }

        return match self.datatype {
            DataEnum::Email => (config.email_length.0 as usize..=config.email_length.1 as usize).contains(&self.login.len()),
            DataEnum::Login => (config.login_length.0 as usize..=config.login_length.1 as usize).contains(&self.login.len()),
            DataEnum::Number => (config.number_length.0 as usize..=config.number_length.1 as usize).contains(&self.login.len()),
            _ => false
        }
    }

    fn new() -> TextValidator {
        TextValidator {
            url: String::new(),
            port: None,
            login: String::new(),
            password: String::new(),
            datatype: DataEnum::NonUsed,
            linetype: LineEnum::Default
        }
    }

    pub fn validate(line: &str, config: &Config, regex_tuple: &(&Regex, &Regex, &Regex), filter_vector: &Vec<String>) -> TextResult {
        let mut validator = TextValidator::new();
        match validator.get_parts_in_line(line) {
            Some(result) => {
                validator.login = result.login;
                validator.password = result.password;
                validator.url = result.url;
                validator.port = result.port;
                validator.linetype = result.linetype;
                validator.datatype = validator.find_type_credits(config, regex_tuple);
                if validator.checking_bad_words_in_credits(filter_vector) {
                    return TextResult::BadWordInCreditError
                };
                if !validator.validate_credit_length(config) {
                    return TextResult::LengthCreditError
                };
                if !validator.validate_full_length(line, config) {
                    return TextResult::LengthAllError
                };
                return TextResult::TextValidator(validator)
            },
            None => TextResult::NotUlp
        }
    }
    pub fn credits(&self) -> String {
        format!("{}:{}", self.login, self.password)
    }
    pub fn urllogpass(&self) -> String {
        if let Some(port) = self.port {
            return format!("https://{}:{}:{}:{}", self.url, port, self.login, self.password);
        }
        format!("https://{}:{}:{}", self.url, self.login, self.password)
    }
}