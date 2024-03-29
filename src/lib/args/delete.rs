use crossterm::style::Color;

use crate::{helpers, models::args, password::db_password, print};

use super::args::Args;

///Usage:
///pm delete "PASSWORD-NAME"
///
///or
///
///pm delete --all
pub struct Delete {
    arguments: Args,
}
//Implementation for Delete struct
impl Delete {
    ///It delete password by name
    ///
    /// If user want to delete all password it run function
    fn delete_password(&self, connection: &sqlite::Connection) {
        let arg = self.arguments.arguments(2).unwrap();

        if arg == "--all" {
            return self.delete_all(connection);
        }
        if !helpers::confirm(Color::Red, &format!("It will delete {} !", &arg)) {
            return;
        }
        match db_password::delete::delete_one_password(arg, connection) {
            Ok(_) => helpers::print_with_color_line(Color::Green, "Password deleted"),
            Err(err) => helpers::print_with_color_line(Color::Red, &err),
        };
    }

    ///It delete all passwords that recorded in database
    fn delete_all(&self, connection: &sqlite::Connection) {
        if !helpers::confirm(Color::Red, "It will delete all passwords!") {
            return;
        }
        if !helpers::confirm(Color::Red, "Are you sure you want to do this") {
            return;
        }
        match db_password::delete::delete_all_passwords(connection) {
            Ok(_) => helpers::print_with_color_line(Color::Green, "Passwords deleted"),
            Err(err) => helpers::print_with_color_line(Color::Red, &err),
        };
    }
}

///Trait implementation for Delete
impl args::Arguments for Delete {
    ///Return instance for Delete struct
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }

    ///It runs the delete password
    ///
    ///If password name not specify from user the get the password name from cli
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

    ///It check second argument which can be "help", "example", "PASSWORD-NAME" or "--all"
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

    ///It runs help function for delete argument
    fn help(&self) {
        print::delete::print_add_help();
    }

    ///It runs help function first then runs example for delete argument
    fn example(&self) {
        self.help();
        print::delete::print_add_example()
    }
}
