use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Read, Write},
    path::PathBuf,
    ffi::OsStr,
};

use smallvec::SmallVec;

use crate::{logo::print_logo, system::clear_screen, FILTER_PATH, ZAPROS_PATH};


pub struct BodySettings {
    pub zapros: Vec<Vec<u8>>,
    pub filter: Vec<Vec<u8>>,
    pub writers: HashMap<String, RefCell<BufWriter<File>>>,
}

impl BodySettings {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            zapros: Self::load_body(&ZAPROS_PATH)?,
            filter: Self::load_body(&FILTER_PATH)?,
            writers: HashMap::new(),
        })
    }

    fn load_body(path: &PathBuf) -> std::io::Result<Vec<Vec<u8>>> {
        let file = OpenOptions::new().read(true).open(path)?;
        Ok(BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok().map(|l| l.into_bytes()))
            .collect::<Vec<_>>())
    }    
}

pub struct LoaderFiles {
    path: PathBuf,
    files: Vec<PathBuf>,
}

impl LoaderFiles {
    pub fn new(path: &PathBuf) -> std::io::Result<Self> {
        let mut loader = Self {
            path: path.clone(),
            files: Vec::new(),
        };
        loader.load_files_recursively(None)?;
        loader.files.reverse();
        Ok(loader)
    }

    fn load_files_recursively(&mut self, dir: Option<PathBuf>) -> std::io::Result<()> {
        let dir = dir.unwrap_or_else(|| self.path.clone());
        if dir.is_file() {
            self.files.push(dir);
            return Ok(());
        }

        for entry in dir.read_dir()? {
            let path = entry?.path();
            if path.is_file() && path.extension() == Some(OsStr::new("txt")) {
                self.files.push(path);
            } else if path.is_dir() {
                self.load_files_recursively(Some(path))?;
            }
        }
        Ok(())
    }

    pub fn init_file(path: &PathBuf) -> std::io::Result<u64> {
        File::open(path).and_then(|file| file.metadata().map(|meta| meta.len()))
    }

    pub fn get_path() -> std::io::Result<PathBuf> {
        loop {
            let mut path = String::new();
            clear_screen()?;
            print_logo();

            print!("[Path]=> ");
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut path)?;

            path = path.trim()
                .replace("& '", "")
                .replace("'", "")
                .replace("\"", "");

            let link_path = PathBuf::from(&path);

            if link_path.exists() {
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

pub struct LoaderBody {
    reader: BufReader<File>,
    buffer: SmallVec<[u8; 1024]>
}

impl LoaderBody {
    pub fn new(file: PathBuf) -> std::io::Result<Self> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        Ok(Self {
            reader,
            buffer: SmallVec::new()
        })
    }

    fn load_body(&mut self) -> Option<(SmallVec<[u8; 1024]>, usize)> {
        self.buffer.clear();
        let mut temp_buffer = SmallVec::<[u8; 1024]>::new();
        temp_buffer.resize(1024, 0);

        match self.reader.read(temp_buffer.as_mut_slice()) {
            Ok(0) => None,
            Ok(n) => {
                self.buffer.extend_from_slice(&temp_buffer[..n]);
                Some((self.buffer.clone(), n))
            }
            Err(_) => None,
        }
    }
}

impl Iterator for LoaderBody {
    type Item = (SmallVec<[u8; 1024]>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.load_body()
    }
}
