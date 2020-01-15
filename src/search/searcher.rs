use std::io::{ Write, BufRead, BufReader, Read };
use std::fs::{
    self,
    File,
    OpenOptions,
};
use std::path::Path;
use std::time::Instant;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

//use grep::matcher::Matcher;
use grep::searcher::sinks;
use grep::regex::RegexMatcher;
use grep::searcher::{Searcher, SearcherBuilder, BinaryDetection};
use grep::searcher::sinks::UTF8;
use grep::printer::Standard;

use rand;
use rand::seq::SliceRandom;

use std::thread;

pub struct MatchInfo {
    pub filename: String,
    pub line: String,
    pub line_number: u64,
}

pub struct ParallelSearcher {
    pub threads: usize,

    pub searchers: Vec<Searcher>,
    pub split_paths: Vec<Vec<String>>,

    pub results: Vec<MatchInfo>,
}

impl ParallelSearcher {
    pub fn new(mut paths: Vec<String>, threads: usize) -> Self {
        let mut searcher = SearcherBuilder::new()
            .binary_detection(BinaryDetection::quit(b'\x00'))
            .build();

        let mut searchers = vec![];
        for i in 0..threads {
            searchers.push(searcher.clone());
        }

        // Shuffle because in our test data later paths do more.
        paths.shuffle(&mut rand::thread_rng());
        dbg!(paths.len());

        let mut split_paths = vec![];
        for chunk in paths.chunks((paths.len() + 1) / threads) {
            split_paths.push(chunk.to_owned());
        }
        dbg!(split_paths.len());

        Self {
            threads,
            searchers,
            split_paths,
            results: vec![],
        }
    }

    pub fn search(&mut self, query: &str) {
        let now = Instant::now();

        let matcher = {
            match RegexMatcher::new(query) {
                Ok(m) => m,
                Err(e) => return,
            }
        };

        self.results.clear();

        let (send, recv) = channel();
        let mut children = Vec::with_capacity(self.threads);

        for i in 0..self.threads {
            let sx = send.clone();

            let matchr = matcher.clone();

            let mut searcher = self.searchers.pop().unwrap();
            let paths = self.split_paths.pop().unwrap();

            let child = thread::spawn(move || {
                paths.iter()
                    .for_each(|path| {
                        searcher.search_path(
                            &matchr,
                            path,
                            UTF8(|lnum, line| {
                                let match_info = MatchInfo {
                                    filename: path.clone(),
                                    line: line.to_string(),
                                    line_number: lnum,
                                };

                                sx.send(match_info);
                                Ok(true)
                            }),
                        );
                    });
            });

            children.push(child);
        }

        for child in children {
            child.join().expect("Failed to join.");
            dbg!("Joined.");
        }

        while let Ok(match_info) = recv.try_recv() {
            self.results.push(match_info);
        }

        let elapsed = now.elapsed();
        dbg!(elapsed);
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
