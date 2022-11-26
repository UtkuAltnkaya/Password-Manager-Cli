use crossterm::style::Color;

use crate::{helpers, models::args, password::db_password, print};

use super::args::Args;

pub struct Delete {
    arguments: Args,
}
//pm delete github
impl Delete {
    fn delete_password(&self, connection: &sqlite::Connection) {
        let arg = self.arguments.arguments(2).unwrap();

        if arg == "--all" {
            return self.delete_all(connection);
        }
        if !helpers::confirm(
            Color::Red,
            String::from("It will delete ".to_owned() + &arg + "!"),
        ) {
            return;
        }
        match db_password::delete::delete_one_password(arg, connection) {
            Ok(_) => helpers::print_with_color_line(Color::Green, String::from("Password deleted")),
            Err(err) => helpers::print_with_color_line(Color::Red, err),
        };
    }

    fn delete_all(&self, connection: &sqlite::Connection) {
        if !helpers::confirm(Color::Red, String::from("It will delete all passwords!")) {
            return;
        }
        if !helpers::confirm(Color::Red, String::from("Are you sure you want to do this")) {
            return;
        }
        match db_password::delete::delete_all_passwords(connection) {
            Ok(_) => {
                helpers::print_with_color_line(Color::Green, String::from("Passwords deleted"))
            }
            Err(err) => helpers::print_with_color_line(Color::Red, err),
        };
    }
}

impl args::Arguments for Delete {
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }

    fn run(&mut self, connection: &sqlite::Connection) {
        if self.arguments.get_len() == 1 {
            self.arguments
                .get_from_console("Enter password name to delete:");
        }
        if self.check_second_arg() {
            return;
        }
        self.delete_password(connection)
    }

    fn check_second_arg(&self) -> bool {
        let arg = self.arguments.arguments(2).unwrap();

        if arg == "--help" || arg == "-h" || arg == "-help" || arg == "--help" {
            self.help();
            return true;
        }
        if arg == "-e" || arg == "--e" || arg == "--example" || arg == "-example" {
            self.example();
            return true;
        }
        return false;
    }

    fn help(&self) {
        print::delete::print_add_help();
    }

    fn example(&self) {
        self.help();
        print::delete::print_add_example()
    }
}
