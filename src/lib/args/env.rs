use std::fs;

use crossterm::style::Color;

use super::args::Args;
use crate::{helpers, models::args, print};

pub struct Env {
    arguments: Args,
}

impl Env {
    fn env_run(&self) {
        let arg = self.arguments.arguments(2).unwrap().to_lowercase();

        if arg == "set" {
            self.set_env_secret_key();
        }

        if arg == "get" {
            helpers::print_with_color_and_bold_line(
                Color::Yellow,
                std::env::var("SECRET_KEY").unwrap(),
            );
        }
    }

    fn set_env_secret_key(&self) {
        if !helpers::confirm(Color::Red,String::from(
        "After set secret key all the password will delete and old secret key will not be access",
    )) {
        return;
    }

        let input = helpers::input_and_output(Color::Grey, "Enter new secret key:");
        fs::write(".env", format!("SECRET_KEY=\"{}\"", input)).unwrap();
    }
}

//pm env set
impl args::Arguments for Env {
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }

    fn run(&mut self, _: &sqlite::Connection) {
        if self.arguments.get_len() == 1 {
            self.arguments.get_from_console("Enter env commend:");
        }
        if self.check_second_arg() {
            return;
        };
        self.env_run();
    }

    fn check_second_arg(&self) -> bool {
        let arg = self.arguments.arguments(2).unwrap();
        if arg == "--help" || arg == "-h" || arg == "-help" {
            self.help();
            return true;
        }
        if arg == "-e" || arg == "--example" {
            self.example();
            return true;
        }
        return false;
    }

    fn help(&self) {
        print::env::print_env_help();
    }

    fn example(&self) {
        self.help();
        print::env::print_env_example();
    }
}
