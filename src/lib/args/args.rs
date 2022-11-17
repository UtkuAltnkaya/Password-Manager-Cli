use std::{
    io::{self, Write},
    ops::Index,
};

use super::{add::Add, list};
use crate::help;

#[derive(Default, Clone)]

pub struct Args {
    arguments: Vec<String>,
    len: usize,
}

impl Args {
    pub fn new() -> Self {
        let items = Args::get_arguments();
        Self {
            arguments: items.0,
            len: items.1,
        }
    }

    pub fn run(&mut self, connection: &sqlite::Connection) {
        if self.len == 0 {
            return help::show();
        }

        match self.arguments.index(1).to_lowercase().as_str() {
            "add" => Add::new(self.clone()).run(connection),
            "show" => {}
            "list" => list::lists_password(),
            "--help" | "help" => help::show(),
            _ => {}
        };
    }

    fn get_arguments() -> (Vec<String>, usize) {
        let vec: Vec<String> = std::env::args().collect();
        let len = vec.len() - 1;
        return (vec, len);
    }

    pub fn get_from_console(&mut self, print_line: &str) {
        let mut line: String = String::new();
        print!("{}", print_line);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        self.arguments.insert(2, line);
        self.len += 1;
    }

    pub fn arguments(&self, index: usize) -> String {
        if index > self.len {
            return "".to_owned();
        }
        self.arguments.index(index).to_string()
    }

    pub fn get_len(&self) -> usize {
        self.len
    }
}
