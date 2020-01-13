#![allow(dead_code)]

use std::io::{self, Write, BufRead, Cursor, Read};
use std::fs;
use std::path::Path;
use std::time::Instant;

mod search;
use search::searcher::{ Searcher, SearchFile };

use iced::{
    button,
    Button,
    Column,
    Element,
    Application,
    Command,
    Settings,
    Align,
    Text,
    TextInput,
    text_input
};

fn main() -> Result<(), io::Error> {
    Search::run(Settings::default());
    Ok(())
}

enum State {
    Idle,
}

struct Search<'a> {
    query: String,

    state: State,

    searcher: Searcher<'a>,

    input: text_input::State,
}

impl Default for Search<'_> {
    fn default() -> Self {
        let mut searcher = Searcher::default();
        searcher.init();

        Self {
            query: "".to_string(),
            state: State::Idle,
            searcher,
            input: text_input::State::new(),
        }
    }
}

use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;

impl Search<'_> {
    /*
    pub fn filter_paths_scout(&mut self, with: &[char]) {
        let now = Instant::now();
        // -------------
        self.search_results.clear();
        // --
        let mut strs: Vec<&str> = self.search_include.iter().map(|x| x.as_str()).collect();
        // -- 500us

        let scout = Scout::new(strs);
        let choices = scout.explore(with);

        // --
        let mut results = choices.into_iter().map(|choice| choice.to_string()).collect();
        self.search_results.append(&mut results);
        // -- 7~ ms




        // ------------- 500ms - 1s.
        let elapsed_scout = now.elapsed();
        dbg!(elapsed_scout);
    }
    */

    pub fn filter_paths(&mut self, with: &str) {
        /*
        let now = Instant::now();

        let mut open_opts = OpenOptions::new();
        open_opts.read(true);

        self.search_results.clear();

        let mut i = 0;
        while i < self.search_include.len() {
            let path = &self.search_include[i];

            let mut interesting = false;

            // Check straight up filename first.
            if path.contains(with) {
                self.search_results.push(path.clone());
                interesting = true;
            }

            let file = open_opts.open(path).expect("Failed to open file.");
            let buf_reader = BufReader::new(file);
            for line in buf_reader.lines() {
                if line.is_err() {
                    // Leave this file.
                    break;
                }

                let l = line.unwrap();
                if l.contains(with) {
                    self.search_results.push(l);
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
        */
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

impl Application for Search<'_> {
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A simple counter")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(query) => {
                if query.len() < self.query.len() {
                    //self.search_include = self.search_files.clone();
                    self.searcher.reset();
                } else {
                    //self.filter_paths(query.chars().collect::<Vec<char>>().as_ref());
                    self.searcher.filter_further(&query);
                }

                self.query = query;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let mut results = Column::new()
            .padding(20);

        let len = self.searcher.search_results.len();
        for idx in &self.searcher.search_results[..len.min(20)] {
            let search_file = &self.searcher.retrieve(*idx);

            let text_filename = Text::new(&search_file.filename);
            let text_lines = Text::new(&search_file.lines[0]);

            let mut file_block = Column::new().padding(5);
            file_block = file_block.push(text_filename);
            file_block = file_block.push(text_lines);



            results = results.push(file_block);
        }

        Column::new()
            .padding(20)
            .push(
                TextInput::new(
                    &mut self.input,
                    "Query",
                    &self.query,
                    Message::InputChanged,
                )
                .padding(15)
                .size(30)
            )
            .push(results)
            .into()
    }
}























