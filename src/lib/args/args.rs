use std::ops::Index;

use crossterm::style::Color;

use super::{add::Add, delete::Delete, env::Env, list, show::Show, update::Update};
use crate::{
    helpers,
    models::{args::Arguments, menu::Menu},
    print,
};
///It is the struct that stores arguments and size of arguments
#[derive(Default, Clone)]
pub struct Args {
    arguments: Vec<String>,
    len: usize,
}

impl Args {
    ///It gets arguments which is entered by user
    ///
    ///Returns instance of Args structs
    pub fn new() -> Self {
        let items = Args::get_arguments_from_user();
        Self {
            arguments: items.0,
            len: items.1,
        }
    }
    ///It will get arguments from user
    ///store in vector
    /// returns vector and its size
    fn get_arguments_from_user() -> (Vec<String>, usize) {
        let vec: Vec<String> = std::env::args().collect();
        let len = vec.len() - 1;
        (vec, len)
    }

    ///It runs relevant functions for arguments
    ///
    ///If any argument is entered then it runs "help"
    pub fn run(self, connection: &sqlite::Connection) {
        if self.len == 0 {
            return Menu::new().run(connection);
        }
        match self.arguments(1).unwrap().as_str() {
            "add" => Add::new(self.clone()).run(connection),
            "show" => Show::new(self.clone()).run(connection),
            "list" | "ls" => list::lists_password(connection),
            "menu" => Menu::new().run(connection),
            "update" => Update::new(self.clone()).run(connection),
            "delete" => Delete::new(self.clone()).run(connection),
            "env" => Env::new(self.clone()).run(connection),
            "-h" | "--h" | "--help" | "-help" | "help" => print::help::display_help(),
            "-v" | "--v" | "--version" | "-version" | "version" => print::display_version(),
            _ => {}
        };
    }
    ///If user not enter for ("add","show",...)
    /// get that argument from cli
    pub fn get_from_console(&mut self, print_line: &str) {
        let line = helpers::input_and_output(Color::Grey, print_line);
        self.insert_arguments(2, line).unwrap();
    }

    ///It returns argument
    pub fn arguments(&self, index: usize) -> Result<String, String> {
        if index > self.len {
            return Err(("Out of range").to_owned());
        }
        Ok(self.arguments.index(index).to_string())
    }

    ///Insert new arg the the arguments vector
    pub fn insert_arguments(&mut self, index: usize, element: String) -> Result<(), String> {
        if element == "" {
            return Err("Invalid item".to_owned());
        }
        self.arguments.insert(index, element);
        self.len += 1;
        Ok(())
    }

    ///It will replace old argument to new argument
    pub fn replace_argument(&mut self, index: usize, element: String) -> Result<(), String> {
        if element == "" {
            return Err(String::from("Invalid item"));
        }
        if index > self.len {
            return Err(String::from("Out of range"));
        }

        self.arguments[index] = element;
        Ok(())
    }
    ///Returns size of arguments vector
    pub fn get_len(&self) -> usize {
        self.len
    }
}
