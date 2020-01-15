#![allow(dead_code)]

use std::io::{self, Write, BufRead, Cursor, Read};
use std::fs;
use std::path::Path;
use std::time::Instant;

mod search;
use search::searcher::Searcher;

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
    searcher: Searcher,
    input: text_input::State,
}

impl Default for Search {
    fn default() -> Self {
        let searcher = Searcher::new("./");

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
                    self.searcher.reset_filter();
                } else {
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

        //let len = self.searcher.search_results.len();
        let mut view_lines = 0;
        for (idx, lines) in &self.searcher.search_results {
            let search_file = &self.searcher.search_base[*idx];

            let mut file_block = Column::new().padding(5);
            file_block = file_block.push(Text::new(&search_file.filename));
            view_lines += 1;

            for line_num in lines {
                let mut row = Row::new().spacing(10);
                row = row.push(Text::new(line_num.to_string()).width(Length::Shrink));
                row = row.push(Text::new(&search_file.lines[*line_num]));
                file_block = file_block.push(row);

                view_lines += 1;
            }

            results = results.push(file_block);

            if view_lines >= 20 {
                break;
            }
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























