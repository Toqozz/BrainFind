use std::io::{ Write, BufRead, BufReader, Read };
use std::fs::{
    self,
    File,
    OpenOptions,
};
use std::path::Path;
use std::time::Instant;
use std::sync::mpsc::channel;

//use grep::matcher::Matcher;
use grep::searcher::sinks;
use grep::regex::RegexMatcher;
use grep::searcher::{Searcher, SearcherBuilder, BinaryDetection};
use grep::searcher::sinks::UTF8;
use grep::printer::Standard;

use ignore::WalkBuilder;

pub struct MatchInfo {
    pub filename: String,
    pub line: String,
    pub line_number: u64,
}

pub fn search(query: &str, paths: &Vec<String>) -> Vec<MatchInfo> {
    let now = Instant::now();

    let matcher = RegexMatcher::new(query).expect("Bad regex.");
    let parallel_walker = WalkBuilder::new("./")
        .standard_filters(false)
        .threads(16)
        .build_parallel();

    let searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .build();

    let (sx, rx) = channel();

    parallel_walker.run(|| {
        let sx = sx.clone();
        let matcher = matcher.clone();
        let mut search = searcher.clone();

        Box::new(move |entry| {
            let entry = match entry {
                Err(err) => {
                    println!("{}", err);
                    return ignore::WalkState::Continue;
                }
                Ok(dent) => {
                    if !dent.file_type().map_or(false, |ft| ft.is_file()) {
                        return ignore::WalkState::Continue;
                    }
                    dent
                }
            };

            //let match_count = 0;
            let mut match_file = String::from("placeholder file");
            let mut match_str = String::from("placeholder str");
            let mut match_line = 0;
            let mut found = false;
            let result = search.search_path(
                &matcher,
                entry.path(),
                UTF8(|lnum, line| {
                    // TODO: make this not so bad.
                    match_file = entry.path().to_str().unwrap().to_string();
                    match_str = line.to_string();
                    match_line = lnum;
                    found = true;
                    Ok(true)
                }),
            );

            if let Err(err) = result {
                println!("{}: {}", entry.path().display(), err);
            } else if found {
                sx.send(MatchInfo { filename: match_file, line: match_str, line_number: match_line });
            }

            ignore::WalkState::Continue
        })
    });

    drop(sx);

    let mut matches = vec![];
    let mut it = rx.iter();
    while let Some(val) = it.next() {
        matches.push(val);
    }

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
