use std::{
    io::{self, Write},
    ops::Index,
};

use crate::help;

use super::add;

#[derive(Default, Clone)]

pub struct Args {
    arguments: Vec<String>,
    len: usize,
}

impl Args {
    pub fn new() {
        let mut obj: Args = Args::default();
        obj.get_arguments();
        obj.run();
    }

    pub fn get_arguments(&mut self) {
        self.arguments = std::env::args().collect();
        self.len = self.arguments.len() - 1;
    }

    pub fn run(&mut self) {
        if self.len == 0 {
            return help::show();
        }
        match self.arguments.index(1).to_lowercase().as_str() {
            "add" => return add::Add::new(self.clone()),
            "show" => {}
            "--help" | "help" => return help::show(),
            _ => {}
        }
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
