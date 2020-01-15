#![allow(dead_code)]

use std::io::{self, Write, BufRead, Cursor, Read};
use std::fs;
use std::path::Path;
use std::time::Instant;

mod search;
use search::searcher;

use iced::{
    button,
    Button,
    Column,
    Row,
    Length,
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

struct Search {
    query: String,

    state: State,

    //files: Vec<String>,
    searcher: searcher::ParallelSearcher,
    //results: Vec<MatchInfo>,

    input: text_input::State,
}

impl Default for Search {
    fn default() -> Self {
        //let mut searcher = Searcher::default();
        //searcher.init();

        let files = searcher::visit_dirs("./".as_ref());
        let searcher = searcher::ParallelSearcher::new(files, 1);

        Self {
            query: "".to_string(),
            state: State::Idle,
            //files,
            searcher,
            //results: vec![],
            //searcher,
            input: text_input::State::new(),
        }
    }
}

use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use crate::search::searcher::MatchInfo;

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

impl Application for Search {
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Brain Find")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(query) => {
                if query.len() < self.query.len() {
                } else {
                    self.searcher.search(&query);
                    //let results = searcher::search(&query, &mut self.files.clone());
                }

                self.query = query;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let mut results = Column::new()
            .padding(20);

        let len = self.searcher.results.len();
        for match_info in &self.searcher.results[..len.min(20)] {
            let text_filename = Text::new(&match_info.filename);
            let text_line = Text::new(&match_info.line).width(Length::Fill);
            let text_line_num = Text::new(&match_info.line_number.to_string()).width(Length::Shrink);

            let mut file_block = Column::new().padding(5);
            file_block = file_block.push(text_filename);
            let mut row = Row::new().spacing(10);
            row = row.push(text_line_num);
            row = row.push(text_line);
            file_block = file_block.push(row);

            results = results.push(file_block);
        }

        /*
        for idx in &self.searcher.search_results[..len.min(20)] {
            let search_file = &self.searcher.retrieve(*idx);

            let text_filename = Text::new(&search_file.filename);
            let text_lines = Text::new(&search_file.lines[0]);

            let mut file_block = Column::new().padding(5);
            file_block = file_block.push(text_filename);
            file_block = file_block.push(text_lines);



            results = results.push(file_block);
        }
        */

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























