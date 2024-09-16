use std::{fs::create_dir, path::{Path, PathBuf}};

use crate::system::get_current_data_path;

pub fn make_result_dir() -> PathBuf {
    let main_path = Path::new("Results");
    if !main_path.is_dir() {
        if let Err(e) = create_dir(main_path) {
            panic!("Не удалось создать папку Results: {}", e);
        }
    }
    let current_date_path = main_path.join(get_current_data_path());

    if !current_date_path.is_dir() {
        if let Err(e) = create_dir(current_date_path.clone()) {
            panic!("Не удалось создать папку Results: {}", e);
        }
    }

    current_date_path.to_path_buf()
}