use std::{collections::{HashMap, HashSet}, sync::mpsc, thread::{self, JoinHandle}};

use regex::Regex;

use crate::{config::Config, schemes::{DataEnum, TextResult, TextValidator, ThreadResult}};


impl ThreadResult {
    pub fn new() -> Self {
        ThreadResult {
            result: HashMap::new(),
            bad_word: 0,
            length_all: 0,
            length_credit: 0,
            not_ulp: 0,
            regex_error: 0,
            total_count: 0
        }
    }

    pub fn start_threads(buffer: &HashMap<String, Vec<String>>, config: &Config, regex_tuple: (Regex, Regex, Regex), filter_vector: &Vec<String>, threads: usize) -> ThreadResult {
        let mut result = ThreadResult::new();

        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        let (sender, receiver) = mpsc::channel::<ThreadResult>();

        for chunk in split_data(buffer.clone(), threads) {
            let sender = sender.clone();
            let regex_tuple = regex_tuple.clone();
            let config = config.clone();
            let filter_vector = filter_vector.to_owned();
            let chunk = chunk.clone();
            let handle = thread::spawn(move || {
                let mut result_schema: ThreadResult = ThreadResult::new();
                for (key, value) in chunk {
                    for line in value {
                        let result_text: TextResult = TextValidator::validate(&line, &config, &(&regex_tuple.0, &regex_tuple.1, &regex_tuple.2), &filter_vector);
                        match result_text {
                            TextResult::BadWordInCreditError => result_schema.bad_word += 1,
                            TextResult::LengthAllError => result_schema.length_all += 1,
                            TextResult::LengthCreditError => result_schema.length_credit += 1,
                            TextResult::NotUlp => result_schema.not_ulp += 1,
                            TextResult::RegexError => result_schema.regex_error += 1,
                            TextResult::TextValidator(text_validator) => {
                                let new_key = match text_validator.datatype {
                                    DataEnum::Email => "email",
                                    DataEnum::Login => "login",
                                    DataEnum::Number => "number",
                                    _ => "default"
                                };
                                if key == "default_data" {
                                    result_schema.result.entry(new_key.to_string()).or_insert(vec![]).push(text_validator);
                                }
                                else {
                                    result_schema.result.entry(format!("{}_{}", key, new_key)).or_insert(vec![]).push(text_validator);
                                }
                            }
                        }
                    }
                }
                sender.send(result_schema).unwrap();
            });
            handles.push(handle);
        }
        drop(sender);
        for receiver in receiver.iter() {
            result.result.extend(receiver.result);
            result.bad_word += receiver.bad_word;
            result.length_all += receiver.length_all;
            result.length_credit += receiver.length_credit;
            result.not_ulp += receiver.not_ulp;
            result.regex_error += receiver.regex_error;
        }
        for handle in handles {
            handle.join().unwrap();
        }

        result
    }
}


fn split_data(
    input: HashMap<String, Vec<String>>, 
    threads: usize
) -> Vec<HashMap<String, Vec<String>>> {
    let mut result: Vec<HashMap<String, Vec<String>>> = vec![HashMap::new(); threads];
    
    let mut used_strings: HashSet<String> = HashSet::new();
    
    for (key, values) in input {
        let unique_values: Vec<String> = values.into_iter()
                                               .filter(|v| !used_strings.contains(v))
                                               .collect();
        let total_values = unique_values.len();
        
        let threads = threads.min(total_values);

        let chunk_size = if threads > 0 {
            (total_values + threads - 1) / threads
        } else {
            1
        };

        for (i, chunk) in unique_values.chunks(chunk_size).enumerate() {
            let thread_map = result.get_mut(i).unwrap();
            thread_map.entry(key.clone())
                      .or_insert_with(Vec::new)
                      .extend(chunk.iter().cloned());
            
            used_strings.extend(chunk.iter().cloned());
        }
    }

    result
}