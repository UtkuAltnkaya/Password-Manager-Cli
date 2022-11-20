use std::ops::Index;

use super::{add::Add, list, show::Show};
use crate::{helpers, models::args::Arguments, print};

#[derive(Default, Clone)]
pub struct Args {
    arguments: Vec<String>,
    len: usize,
}

impl Args {
    pub fn new() -> Self {
        let items = get_arguments_from_user();
        Self {
            arguments: items.0,
            len: items.1,
        }
    }

    pub fn run(self, connection: &sqlite::Connection) {
        if self.len == 0 {
            return print::help::show();
        }
        match self.arguments(1).unwrap().as_str() {
            "add" => Add::new(self.clone()).run(connection),
            "show" => Show::new(self.clone()).run(connection),
            "list" | "ls" => list::lists_password(connection),
            "--help" | "help" => print::help::show(),
            _ => {}
        };
    }

    pub fn get_from_console(&mut self, print_line: &str) {
        let line = helpers::input_and_output(print_line);
        self.insert_arguments(2, line).unwrap();
    }

    pub fn arguments(&self, index: usize) -> Result<String, String> {
        if index > self.len {
            return Err(("Out of range").to_owned());
        }
        Ok(self.arguments.index(index).to_string())
    }

    pub fn insert_arguments(&mut self, index: usize, element: String) -> Result<(), String> {
        if element == "" {
            return Err("Invalid item".to_owned());
        }
        self.arguments.insert(index, element);
        self.len += 1;
        Ok(())
    }

    pub fn get_len(&self) -> usize {
        self.len
    }
}

fn get_arguments_from_user() -> (Vec<String>, usize) {
    let vec: Vec<String> = std::env::args().collect();
    let len = vec.len() - 1;
    (vec, len)
}
