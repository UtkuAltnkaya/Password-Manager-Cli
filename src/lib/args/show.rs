use colored::Colorize;

use crate::{
    helpers,
    models::{args, password::Password},
    password::db_password,
    print,
};

use super::args::Args;

pub struct Show {
    arguments: Args,
}

impl Show {
    fn show_password(&self, connection: &sqlite::Connection) {
        let result =
            db_password::get_one_password(self.arguments.arguments(2).unwrap(), connection);

        match result {
            Ok(password) => self.result(password),
            Err(error) => println!("{}", error.red()),
        }
    }

    fn result(&self, mut password: Password) {
        println!(
            "{:<15} {:<15} {:<15}",
            "Id".yellow(),
            "Name".yellow(),
            "Password".yellow()
        );
        println!(
            "{:<15} {:<15} {:<15}",
            1,
            password.get_password_name(),
            "********"
        );

        let i = helpers::input_and_output("To see password press y:");
        if i != "y" {
            return;
        }
        password.decrypt();
        println!("{}", password.get_password().blue());
    }
}

impl args::Arguments for Show {
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }

    fn run(&mut self, connection: &sqlite::Connection) {
        if self.arguments.get_len() == 1 {
            self.arguments.get_from_console("Enter password to show:");
        }
        if self.check_second_arg() {
            return;
        }
        self.show_password(connection)
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
        print::show::print_show_help()
    }

    fn example(&self) {
        print::show::print_show_example();
    }
}
