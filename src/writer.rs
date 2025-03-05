use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Write}
};

use big_brains_club_logo::LogoBuilder;
use vulp::{ResultVULP, ValidationError};

use crate::{config::Config, CURRENT_DIR, END_FILES_ALL_PARSE, ERROR_FILTER_STR, ERROR_LENGTH_STR, ERROR_LP_EQ_STR, ERROR_LP_STR, ERROR_PARSE_STR, VALID_COUNT_STR};

pub struct Writer {
    bufwriters: HashMap<String, BufWriter<File>>,
}

impl Writer {
    pub fn new() -> Self {
        Self { bufwriters: HashMap::new() }
    }
    fn check_exist(&mut self, key: &str, line: &[u8]) -> std::io::Result<()> {
        let writer = self.bufwriters.entry(key.to_string()).or_insert_with(|| {
            let file_path = CURRENT_DIR.join(key);
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(file_path)
                .expect("Failed to open file");
            BufWriter::new(file)
        });

        writer.write_all(line)?;
        writer.write_all(b"\n")?;
        Ok(())
    }

    pub fn write(&mut self, result: &HashMap<String, Vec<Result<ResultVULP, ValidationError>>>, config: &Config, counter: &mut LogoBuilder) -> std::io::Result<()> {
        for (key, value) in result {
            for line in value {
                match line {
                    Ok(validator) => {
                        counter.entry_extra(VALID_COUNT_STR, Some(1));
                        if config.parse_full {
                            self.check_exist(&format!("{key}_{}.txt", END_FILES_ALL_PARSE), &validator.full_line)?;
                        }
                        self.check_exist(&format!("{key}_{}.txt", validator.datatype), &validator.credits)?;
                    },
                    Err(e) => {
                        match e {
                            ValidationError::FilterError => {
                                counter.entry_extra(ERROR_FILTER_STR, Some(1));
                            },
                            ValidationError::FindDataTypeError => {
                                counter.entry_extra(ERROR_LP_STR, Some(1));
                            },
                            ValidationError::LengthError => {
                                counter.entry_extra(ERROR_LENGTH_STR, Some(1));
                            },
                            ValidationError::ParseError => {
                                counter.entry_extra(ERROR_PARSE_STR, Some(1));
                            },
                            ValidationError::EqualError => {
                                counter.entry_extra(ERROR_LP_EQ_STR, Some(1));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
