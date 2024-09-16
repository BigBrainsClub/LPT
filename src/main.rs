use std::path::Path;

use regex::Regex;
use urllogpass::config::Config;
use urllogpass::folders::make_result_dir;
use urllogpass::loader_paths;
use urllogpass::reading::reading_file;
use urllogpass::schemes::ThreadResult;
use urllogpass::select_path::return_path;
use urllogpass::system::get_threads;

fn main() {
    let (zapros, filter) = urllogpass::loader_settings::load_configuration();

    let config = Config::new().load_config(&Path::new("config.json"));

    let path_work = return_path();
    let path_result = make_result_dir();

    let threads = get_threads(&config);


    let email_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    let login_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    let number_regex = Regex::new(r"^\+?\d{1,4}?[-.\s]?\(?\d{1,3}?\)?[-.\s]?\d{1,4}[-.\s]?\d{1,4}[-.\s]?\d{1,9}$").unwrap();
    
    let paths = loader_paths::load_files_in_path(&path_work);

    let mut main_result = ThreadResult::new();

    match paths {
        Some(paths) => {
            for path in paths {
                let result = reading_file(&path, &config, &zapros, &filter, threads, (email_regex.clone(), login_regex.clone(), number_regex.clone()), &path_result);
                main_result.bad_word += result.bad_word;
                main_result.length_all += result.length_all;
                main_result.length_credit += result.length_credit;
                main_result.not_ulp += result.not_ulp;
                main_result.regex_error += result.regex_error;
                main_result.total_count += result.total_count;
            }
        },
        None => panic!("Путь {} не найден", path_work.display())
    }
    println!("all count find - {}\nbad words - {}\nlength all - {}\nlength credit - {}\nnot ulp - {}\nregex error - {}\n", main_result.total_count, main_result.bad_word, main_result.length_all, main_result.length_credit, main_result.not_ulp, main_result.regex_error);
}