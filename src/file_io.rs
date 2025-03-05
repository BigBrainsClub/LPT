use std::{
    fs::{File, OpenOptions}, io::{BufRead, BufReader, Seek, SeekFrom, Write}, path::PathBuf
};

use smallvec::SmallVec;

use crate::{system::clear_screen, FILTER_PATH, RENDERED_LOGO, ZAPROS_PATH};


pub struct BodySettings {
    pub zapros: SmallVec<[SmallVec<[u8; 16]>; 4]>,
    pub filter: SmallVec<[SmallVec<[u8; 16]>; 4]>,
}

impl BodySettings {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            zapros: Self::load_body(&ZAPROS_PATH)?,
            filter: Self::load_body(&FILTER_PATH)?,
        })
    }

    fn load_body(path: &PathBuf) -> std::io::Result<SmallVec<[SmallVec<[u8; 16]>; 4]>> {
        let file = OpenOptions::new().read(true).open(path)?;
        let mut result = SmallVec::new();

        for line in BufReader::new(file).lines().filter_map(|line| line.ok()) {
            let mut small_vec = SmallVec::<[u8; 16]>::new();
            small_vec.extend_from_slice(line.as_bytes());
            result.push(small_vec);
        }

        Ok(result)
    }

    pub fn load_init() -> std::io::Result<()> {
        Self::init(&ZAPROS_PATH)?;
        Self::init(&FILTER_PATH)?;
        Ok(())
    }

    fn init(path: &PathBuf) -> std::io::Result<()> {
        if !path.exists() {
            File::create(path)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct LoaderFiles {
    path: PathBuf,
    files: Vec<PathBuf>
}

impl LoaderFiles {
    pub fn new(path: &PathBuf) -> std::io::Result<Self> {
        let mut loader = Self {
            path: path.clone(),
            files: Vec::new()
        };
        loader.load_files_recursively(None)?;
        loader.files.reverse();
        Ok(loader)
    }

    pub fn load_files_recursively(&mut self, path: Option<PathBuf>) -> std::io::Result<()> {
        let path = path.unwrap_or_else(|| self.path.clone());

        for entry in walkdir::WalkDir::new(&path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file() && e.path().extension() == Some(std::ffi::OsStr::new("txt")))
        {
            self.files.push(entry.path().to_path_buf());
        }

        Ok(())
    }


    pub fn init_file(path: &PathBuf) -> std::io::Result<u64> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::End(0))
    }
    pub fn get_path() -> std::io::Result<PathBuf> {
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();
        loop {
            let mut path = String::new();
            clear_screen()?;
            println!("{}", *RENDERED_LOGO);

            print!("[Path]=> ");
            stdout.flush()?;
            stdin.read_line(&mut path)?;

            path = path.trim()
                .replace("& '", "")
                .replace("'", "")
                .replace("\"", "");
            
            let link_path = if path == "." {
                std::env::current_dir()?
            } else {
                PathBuf::from(&path)
            };
            if link_path.exists() {
                clear_screen()?;
                return Ok(link_path);
            }

            println!("Путь {} не найден", path);
            std::io::stdin().read_line(&mut String::new())?;
        }
    }
}

impl Iterator for LoaderFiles {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.files.pop()
    }
}