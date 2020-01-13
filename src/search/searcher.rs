use std::io::{ Write, BufRead, BufReader, Read };
use std::fs::{
    self,
    File,
    OpenOptions,
};
use std::path::Path;
use std::time::Instant;


#[derive(Clone)]
pub struct SearchFile {
    pub filename: String,
    pub lines: Vec<String>,
}

impl SearchFile {
    fn new(path: &Path) -> Self {
        let mut lines = Vec::new();

        let file = OpenOptions::new().read(true).open(path).expect("Failed to open file.");
        let buf_reader = BufReader::new(file);

        for line in buf_reader.lines() {
            if let Ok(line_str) = line {
                lines.push(line_str);
            } else {
                // Probably a binary file or something -- not useful.
                break;
            }
        }

        Self {
            filename: path.to_str().unwrap().to_string(),
            lines,
        }
    }
}

pub struct Searcher<'a> {
    search_dir: &'a Path,

    search_base: Vec<SearchFile>,
    search_include: Vec<usize>,

    pub search_results: Vec<usize>,
}

impl Default for Searcher<'_> {
    fn default() -> Self {
        let search_base = vec![];
        let search_include = vec![];
        let search_results = vec![];

        Self {
            search_dir: "./".as_ref(),
            search_base,
            search_include,
            search_results,
        }
    }
}

impl<'a> Searcher<'a> {
    pub fn init(&mut self) {
        let now = Instant::now();

        self.search_base = visit_dirs(self.search_dir);
        for i in 0..self.search_base.len() {
            self.search_include.push(i);
        }

        let elapsed_visit = now.elapsed();
        dbg!(elapsed_visit);
    }

    pub fn filter_further(&mut self, query: &str) {
        let now = Instant::now();

        self.search_results.clear();

        let mut i = 0;
        while i < self.search_include.len() {
            let idx = self.search_include[i];

            let search_file = &self.search_base[idx];

            let mut interesting = false;
            if search_file.filename.contains(query) {
                self.search_results.push(idx);
                interesting = true;
            }

            for line in &search_file.lines {
                if line.contains(query) {
                    self.search_results.push(idx);
                    interesting = true;
                }
            }

            if !interesting {
                self.search_include.swap_remove(i);
            } else {
                i += 1;
            }
        }

        let elapsed = now.elapsed();
        dbg!(elapsed);
    }

    pub fn reset(&mut self) {
        self.search_include.clear();
        for i in 0..self.search_base.len() {
            self.search_include.push(i);
        }
    }

    pub fn retrieve(&self, idx: usize) -> &SearchFile {
        &self.search_base[idx]
    }
}

fn visit_dirs(dir: &Path) -> Vec<SearchFile> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                files.append(visit_dirs(&entry.path()).as_mut());
            } else if entry.path().is_file() {
                let filename = entry.path().to_str().unwrap().to_string();
                files.push(SearchFile::new(filename.as_ref()));
            }
        }
    }

    files
}