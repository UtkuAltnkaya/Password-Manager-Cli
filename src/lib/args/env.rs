use std::{
    fs::{self, File},
    io::Write,
};

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
                &std::env::var("SECRET_KEY").unwrap(),
            );
        }
    }

    fn set_env_secret_key(&self) {
        let confirm = helpers::confirm(Color::Red,
        "After setting a secret key, all the password will delete and old secret key will not be access");
        if !confirm {
            return;
        }

        let input = helpers::input_and_output(Color::Grey, "Enter new secret key:");
        let path = helpers::exe_location();

        fs::write(path + ".env", format!("SECRET_KEY=\"{}\"", input)).unwrap();
    }

    pub fn read_from_env(path: &str) {
        let result = dotenv::from_path(path);

        if let Err(error) = result {
            if error.not_found() {
                helpers::print_with_color_and_bold_line(Color::Red, ".env file not found");
                if helpers::confirm(Color::Yellow, "Would you like to create env file") {
                    Env::create_env_file(path);
                    Env::read_from_env(path);
                    return;
                }
            }

            helpers::print_with_color_and_bold_line(Color::Red, &error.to_string());
            std::process::exit(1);
        }
    }

    pub fn check_secret_key() {
        let secret_key = std::env::var("SECRET_KEY").unwrap();
        if secret_key == "" {
            helpers::print_with_color_and_bold_line(
                Color::Red,
                "No secret key found default will use (not recommended)!",
            );

            helpers::print_with_color_and_bold(Color::Yellow, "To see how to change secret key:");

            helpers::print_with_color_and_bold_line(Color::Magenta, " pm --help env");
            std::env::set_var(
                "SECRET_KEY",
                "W?Xa8Q?E>7g3A=O)s6n6N8>s6L3P6pZ2V>n-CwSv$F(1_1)BlO[0x5p$x_a4d4u&",
            );
            println!();
        }
    }

    fn create_env_file(path: &str) {
        let mut file = File::create(path).expect("Error encountered while creating file!");
        file.write_all(b"SECRET_KEY=\"\"").unwrap();
    }
}

//pm env set
impl args::Arguments for Env {
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }

    fn run(&mut self, _: &sqlite::Connection) {
        if self.arguments.get_len() == 1 {
            self.arguments.get_from_console("Enter env command:");
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
