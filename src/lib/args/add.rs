use super::args::Args;
use crate::{
    models::{
        args::{self},
        password::Password,
    },
    password::db_password,
    print,
};
use colored::Colorize;

pub struct Add {
    arguments: Args,
}

impl Add {
    fn add_password(&self, second_argument: String, size: usize, connection: &sqlite::Connection) {
        let mut gn_pass = Password::new(second_argument.clone(), size);
        match gn_pass.generate_password() {
            Ok(result) => {
                let db_result = db_password::add_password_to_db(
                    connection,
                    second_argument,
                    gn_pass.get_password().to_string(),
                );
                if let Err(error) = &db_result {
                    return println!("{}", error.red());
                }
                println!("{}", result.green())
            }
            Err(error) => println!("{}", error.red()),
        };
    }

    fn check_third_args(&self) -> usize {
        let mut size: usize = 32;
        if self.arguments.get_len() != 4 {
            return size;
        }
        let third_arguments = self.arguments.arguments(3).unwrap();
        if third_arguments == "-s" || third_arguments == "--size" {
            size = self.arguments.arguments(4).unwrap().parse().unwrap();
        }
        return size;
    }
}

impl args::Arguments for Add {
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }
    fn run(&mut self, connection: &sqlite::Connection) {
        if self.arguments.get_len() == 1 {
            self.arguments
                .get_from_console("Enter password name to add:");
        }
        if self.check_second_arg() {
            return;
        };
        self.add_password(
            self.arguments.arguments(2).unwrap(),
            self.check_third_args(),
            connection,
        );
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
        print::add::print_add_help();
    }

    fn example(&self) {
        self.help();
        print::add::print_add_example();
    }
}
