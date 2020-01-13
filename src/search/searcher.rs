use std::io::{ Write, BufRead, BufReader, Read };
use std::fs::{
    self,
    File,
    OpenOptions,
};
use std::path::Path;
use std::time::Instant;

use rayon::prelude::*;

//use grep::matcher::Matcher;
use grep::regex::RegexMatcher;
use grep::searcher::{Searcher, SearcherBuilder, BinaryDetection};
use grep::searcher::sinks::UTF8;
use grep::printer::Standard;

pub struct MatchInfo {
    pub filename: String,
    pub line: String,
    pub line_number: u64,
}

pub fn search(query: &str, paths: &Vec<String>) -> Vec<MatchInfo> {
    let now = Instant::now();

    let matcher = RegexMatcher::new(query).expect("Bad regex.");
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .build();

    let mut matches = vec![];

    paths
        .iter()
        .for_each(|path| {
            searcher.search_path(
                &matcher,
                path,
                UTF8(|lnum, line| {
                    let match_info = MatchInfo {
                        filename: path.clone(),
                        line: line.to_string(),
                        line_number: lnum,
                    };

                    matches.push(match_info);
                    Ok(true)
                })
            );
        });

    /*
    for path in paths {
        let result = searcher.search_path(
            &matcher,
            path,
            UTF8(|lnum, line| {
                // Find the exact match.
                //let mymatch = matcher.find(line.as_bytes()).unwrap().unwrap();
                matches.push(line.to_string());
                Ok(true)
            })
        );
    }
    */

    let elapsed = now.elapsed();
    dbg!(elapsed);

    matches
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