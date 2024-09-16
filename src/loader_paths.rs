use std::path::{Path, PathBuf};


pub fn load_files_in_path(path: &Path) -> Option<Vec<PathBuf>> {
    if path.is_file() && path.to_string_lossy().ends_with(".txt") {
        return Some(vec![path.to_path_buf()]);
    }

    if path.is_dir() {
        let mut files: Vec<PathBuf> = Vec::new();
        if let Ok(paths) = path.read_dir() {
            for path in paths {
                if let Ok(path) = path {
                    if path.path().is_dir() {
                        if let Some(paths) = load_files_in_path(&path.path()) {
                            files.extend(paths);
                        }
                    }
                    else if path.path().to_string_lossy().ends_with(".txt") && path.path().is_file() {
                        files.push(path.path().to_path_buf());
                    }
                }
            }
        }
        if !files.is_empty() {
            return Some(files)
        }
   }
   None
}