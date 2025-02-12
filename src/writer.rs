use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Write}
};

use crate::{config::Config, text::TextValidator, CURRENT_DIR, END_FILES_ALL_PARSE};

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

    pub fn write(&mut self, result: &HashMap<String, Vec<TextValidator>>, config: &Config) -> std::io::Result<()> {
        for (key, value) in result {
            for line in value {
                if config.parse_full {
                    self.check_exist(&format!("{key}_{}", END_FILES_ALL_PARSE), &line.full_line())?;
                }
                self.check_exist(&format!("{key}_{}", line.datatype), &line.credits())?;
            }
        }

        Ok(())
    }
}
