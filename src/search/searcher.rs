use std::io::{ Write, BufRead, BufReader, Read };
use std::fs::{
    self,
    File,
    OpenOptions,
};
use std::path::Path;
use std::time::Instant;
use std::sync::mpsc::channel;
use std::thread;

use rayon::prelude::*;

use rand;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct SearchFile {
    pub filename: String,
    pub lines: Vec<String>,
}

pub struct Searcher {
    pub search_base: Vec<SearchFile>,
    pub search_include: Vec<usize>,

    pub search_results: Vec<(usize, Vec<usize>)>,
}

impl Searcher {
    pub fn new(path: &str) -> Self {
        let mut filenames = visit_dirs(Path::new(path));

        let mut open_options = OpenOptions::new();

        let search_base: Vec<SearchFile> =
            filenames.iter().map(|filename| {
                let file = open_options.read(true).open(filename).expect("Failed to open file.");
                let buf_lines = BufReader::new(file).lines();

                // TODO: try just pushing.
                let mut lines = vec![];
                for line in buf_lines {
                    if let Ok(line_str) = line {
                        lines.push(line_str);
                    } else {
                        // Probably a binary file or something.
                        break;
                    }
                }

                SearchFile {
                    filename: filename.to_string(),
                    lines,
                }
            }).collect();

        let search_include: Vec<usize> = (0..search_base.len()).collect();

        Self {
            search_base,
            search_include,
            search_results: vec![],
        }
    }

    pub fn filter_further(&mut self, query: &str) {
        let now = Instant::now();

        self.search_results.clear();

        dbg!(self.search_include.len());

        let mut i = 0;
        while i < self.search_include.len() {
            let idx = self.search_include[i];

            let search_file = &self.search_base[idx];
            let mut interesting = false;
            if search_file.filename.contains(query) {
                interesting = true;

                let mut match_lines = vec![];
                for (j, line) in search_file.lines.iter().enumerate() {
                    if line.contains(query) {
                        match_lines.push(j);
                    }
                }

                self.search_results.push((idx, match_lines));
            } else {
                let mut match_lines = vec![];
                for (j, line) in search_file.lines.iter().enumerate() {
                    if line.contains(query) {
                        interesting = true;

                        match_lines.push(j)
                    }
                }

                self.search_results.push((idx, match_lines));
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

    pub fn reset_filter(&mut self) {
        self.search_include = (0..self.search_base.len()).collect();
    }
}

pub fn visit_dirs(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                files.append(visit_dirs(&entry.path()).as_mut());
            } else if entry.path().is_file() {
                let filename = entry.path().to_str().unwrap().to_string();
                files.push(filename);
            }
        }
    }

    files
}
