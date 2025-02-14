use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Write}
};

use crate::{config::Config, counter::Counters, text::schema::{TextValidator, ValidationError}, CURRENT_DIR, END_FILES_ALL_PARSE};

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

    pub fn write(&mut self, result: &HashMap<String, Vec<Result<TextValidator, ValidationError>>>, config: &Config, counter: &mut Counters) -> std::io::Result<()> {
        for (key, value) in result {
            for line in value {
                match line {
                    Ok(validator) => {
                        counter.valid += 1;
                        if config.parse_full {
                            self.check_exist(&format!("{key}_{}.txt", END_FILES_ALL_PARSE), &validator.full_line())?;
                        }
                        self.check_exist(&format!("{key}_{}.txt", validator.datatype), &validator.credits())?;
                    },
                    Err(e) => {
                        match e {
                            ValidationError::FilterError => {
                                counter.filter_error += 1;
                            },
                            ValidationError::FindDataTypeError => {
                                counter.data_error += 1;
                            },
                            ValidationError::LengthError => {
                                counter.length_error += 1;
                            },
                            ValidationError::ParseError => {
                                counter.parse_error += 1;
                            },
                            ValidationError::LpEqualError => {
                                counter.lp_equal += 1;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
