use std::collections::HashMap;

#[derive(PartialEq)]
pub enum DataEnum {
    Login,
    Email,
    Number,
    NonUsed,
    NonValid
}

#[derive(PartialEq)]
pub enum LineEnum {
    Http,
    Android,
    ReversedHttp,
    WithoutHttp,
    Default
}

pub struct SResultUlp {
    pub url: String,
    pub port: Option<u16>,
    pub login: String,
    pub password: String,
    pub datatype: DataEnum,
    pub linetype: LineEnum
}

pub struct TextValidator {
    pub url: String,
    pub port: Option<u16>,
    pub login: String,
    pub password: String,
    pub linetype: LineEnum,
    pub datatype: DataEnum
}

pub enum TextResult {
    TextValidator(TextValidator),
    LengthAllError,
    LengthCreditError,
    BadWordInCreditError,
    RegexError,
    NotUlp
}

pub struct UnzipLineParts {
    pub url: String,
    pub port: Option<u16>,
    pub login: String,
    pub password: String,
    pub linetype: LineEnum,
}

pub struct ThreadResult {
    pub result: HashMap<String, Vec<TextValidator>>,
    pub bad_word: usize,
    pub length_all: usize,
    pub length_credit: usize,
    pub not_ulp: usize,
    pub regex_error: usize,
    pub total_count: usize
}


pub struct ResultMain {
    pub all_count: usize,
    pub bad_word_count: usize,
    pub length_all_count: usize,
    pub length_credit_count: usize,
    pub not_ulp_count: usize,
    pub regex_error_count: usize
}

impl ResultMain {
    pub fn new() -> Self {
        ResultMain {
            all_count: 0,
            bad_word_count: 0,
            length_all_count: 0,
            length_credit_count: 0,
            not_ulp_count: 0,
            regex_error_count: 0
        }
    }
}