use std::{io::Write, path::{Path, PathBuf}};

use crate::logo::Logo;

pub fn return_path() -> PathBuf {
    loop {
        let mut path = String::new();
        Logo::logo();
        print!("[Path]=> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut path).unwrap();

        path = path.trim().replace("& '", "").replace("'", "");
        path.retain(|c| c != '"');

        let link_path = Path::new(&path);
        
        if link_path.exists() {
            return link_path.to_path_buf()
        } else { 
            println!("Путь {} не найден", path.trim());
            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}