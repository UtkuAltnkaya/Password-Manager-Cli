use cli_clipboard::{ClipboardContext, ClipboardProvider};
use console::style;

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
    fn show_password(&mut self, connection: &sqlite::Connection) {
        let result =
            db_password::get_one_password(self.arguments.arguments(2).unwrap(), connection);

        match result {
            Ok(mut password) => {
                self.result(&mut password);
                self.check_third_args(&password);
            }
            Err(error) => println!("{}", style(error).red()),
        }
    }

    fn result(&self, password: &mut Password) {
        println!(
            "{:<15} {:<15} {:<15}",
            style("Id").yellow(),
            style("Name").yellow(),
            style("Password").yellow(),
        );
        println!(
            "{:<15} {:<15} {:<15}\n",
            1,
            password.get_password_name(),
            "********"
        );
        password.decrypt();

        let input = helpers::input_and_output(
            style("To see password press (y):")
                .blue()
                .bold()
                .to_string()
                .as_str(),
        );
        if input.to_lowercase() != "y" {
            return;
        }
        println!("{}", style(password.get_password()).bold().green());
    }

    fn check_third_args(&mut self, password: &Password) {
        let third_arguments = self.arguments.arguments(3);
        match third_arguments {
            Ok(result) => {
                if result == "-c" || result == "--copy" {
                    let mut ctx = ClipboardContext::new().unwrap();
                    ctx.set_contents(password.password.to_owned()).unwrap();
                    println!("{}", style("Password copied to clipboard").bold().green())
                }
            }
            Err(_) => {
                let input = helpers::input_and_output(
                    style("To copy password to clipboard press (y):")
                        .blue()
                        .bold()
                        .to_string()
                        .as_str(),
                );
                if input.to_lowercase() == "y" {
                    self.arguments
                        .insert_arguments(3, String::from("--copy"))
                        .unwrap();
                    self.check_third_args(password);
                }
            }
        }
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
