use std::{collections::HashMap, fs::OpenOptions, io::Write, path::PathBuf};

use crate::schemes::TextValidator;


pub fn write_result(path: &PathBuf, result: &HashMap<String, Vec<TextValidator>>, write_full: bool) -> usize {
    let mut count: usize = 0;
    for (key, value) in result {
        if !value.is_empty() {
            count += value.len();
            if write_full {
                match OpenOptions::new().create(true).write(true).append(true).open(path.join(&format!("{}_full.txt", key))) {
                    Ok(mut body) => {
                        for classbody in value {
                            if let Err(e) = body.write(format!("{}\n", classbody.urllogpass()).as_bytes()) {
                                println!("Ошибка при записи в файл: {}", e)
                            }
                        }
                    },
                    Err(e) => println!("Ошибка при записи в файл: {}", e)
                }
            }
            match OpenOptions::new().create(true).write(true).append(true).open(path.join(&format!("{}.txt", key))) {
                Ok(mut body) => {
                    for classbody in value {
                        if let Err(e) = body.write(format!("{}\n", classbody.credits()).as_bytes()) {
                            println!("Ошибка при записи в файл: {}", e)
                        }
                    }
                },
                Err(e) => println!("Ошибка при записи в файл: {}", e)
            }
        }
    }
    count
}