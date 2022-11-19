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
        let items = get_arguments_from_user();
        Self {
            arguments: items.0,
            len: items.1,
        }
    }
    // pub fn run(&mut self, connection: &sqlite::Connection) {}
    pub fn run(&mut self, connection: &sqlite::Connection) {
        if self.len == 0 {
            return help::show();
        }
        match self.arguments(1).unwrap().as_str() {
            "add" => Add::new(self.clone()).run(connection),
            "show" => {}
            "list" => list::lists_password(connection),
            "--help" | "help" => help::show(),
            _ => {}
        };
    }

    pub fn get_from_console(&mut self, print_line: &str) {
        let mut line: String = String::new();
        print!("{}", print_line);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        self.insert_arguments(2, line).unwrap();
        self.len += 1;
    }

    pub fn arguments(&self, index: usize) -> Result<String, String> {
        if index > self.len {
            return Err(("Out of range").to_owned());
        }
        Ok(self.arguments.index(index).to_string())
    }

    pub fn insert_arguments(&mut self, index: usize, element: String) -> Result<(), String> {
        if index > self.len {
            return Err("Out of range".to_owned());
        }
        self.arguments.insert(index, element);
        Ok(())
    }

    pub fn get_len(&self) -> usize {
        self.len
    }
}

fn get_arguments_from_user() -> (Vec<String>, usize) {
    let vec: Vec<String> = std::env::args().collect();
    let len = vec.len() - 1;
    return (vec, len);
}
