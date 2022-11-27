use crossterm::style::Color;

use super::args::Args;
use crate::{
    helpers,
    models::{args, password::Password},
    password::db_password,
    print,
};

///Usage:
///pm add "PASSWORD-NAME"
///
/// or
///
///pm add "PASSWORD-NAME" -s 64
///
///-s is used for specify size of password
pub struct Add {
    arguments: Args,
}

//Implementation for Add struct
impl Add {
    ///It generate a random password and store in database.
    ///It will occurs an error when same password name record in database.
    fn add_password(&self, second_argument: String, size: usize, connection: &sqlite::Connection) {
        let mut gn_pass = Password::new(&second_argument, size);
        match gn_pass.generate_password() {
            Ok(result) => {
                let db_result = db_password::add_password_to_db(
                    connection,
                    second_argument,
                    gn_pass.get_password().to_string(),
                );

                if let Err(error) = &db_result {
                    return helpers::print_with_color(Color::Red, &error.to_string());
                }
                helpers::print_with_color(Color::Green, &result);
            }
            Err(error) => helpers::print_with_color(Color::Red, &error),
        };
    }
    ///It check the third argument witch is use for specify the size of password
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

///Trait implementation for Add
impl args::Arguments for Add {
    ///It returns instance for Add struct
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }

    ///That runs for add argument.
    ///
    ///If user not enter the password name to add database
    ///it will get the password name from cli.
    ///
    ///If user use "help or example" run relevant functions.
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

    ///It checks for second argument.
    ///Witch can be "help","example" or "PASSWORD-NAME".
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

    ///Runs help for add password.
    fn help(&self) {
        print::add::print_add_help();
    }

    ///Runs help function first
    ///and runs example for add password.
    fn example(&self) {
        self.help();
        print::add::print_add_example();
    }
}
