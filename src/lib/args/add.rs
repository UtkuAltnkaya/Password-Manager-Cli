use super::args::Args;
use crate::{db::password, generate_password::GeneratePassword};
use colored::Colorize;

pub struct Add {
    arguments: Args,
}

impl Add {
    pub fn new(arguments: Args) -> Self {
        Self { arguments }
    }

    pub fn run(&mut self, connection: &sqlite::Connection) {
        if self.arguments.get_len() == 1 {
            self.arguments
                .get_from_console("Enter password name to add:");
        }
        if self.check_second_arg() {
            return;
        };
        self.add_password(
            self.arguments.arguments(2),
            self.check_third_args(),
            connection,
        );
    }

    fn add_password(&self, second_argument: String, size: usize, connection: &sqlite::Connection) {
        let mut gn_pass = GeneratePassword::new(second_argument.clone(), size);
        match gn_pass.generate_password() {
            Ok(result) => {
                password::add_password_to_db(
                    connection,
                    second_argument,
                    gn_pass.get_password().to_string(),
                )
                .unwrap();
                println!("{}", result.green())
            }
            Err(error) => panic!("{}", error.red()),
        };
    }

    fn check_second_arg(&self) -> bool {
        let arg = self.arguments.arguments(2);
        if arg == "--help" || arg == "-h" {
            self.add_help();
            return true;
        }
        if arg == "-e" || arg == "--example" {
            self.add_example();
            return true;
        }
        if arg.starts_with("-") || arg.starts_with("--") {
            return true;
        }
        return false;
    }

    fn check_third_args(&self) -> usize {
        let mut size: usize = 32;
        if self.arguments.get_len() == 4 {
            let third_arguments = self.arguments.arguments(3);
            if third_arguments == "-s" || third_arguments == "--size" {
                size = self.arguments.arguments(4).parse().unwrap();
            }
        }
        return size;
    }

    fn add_help(&self) {
        println!(
            "{}\n pm.exe add {} [--] {}\n",
            "USAGE:".yellow(),
            "[OPTIONS]".green(),
            "[args]",
        );
        println!(
            "{}\n {: <15} Enter site to add password\n",
            "OPTIONS:".yellow(),
            "[SITE-NAME]".green()
        );
        println!(
            "{}\n {:<15} Specify size of password (Multiples of 4 and 4, up to 128)",
            "ARGS:".yellow(),
            "-s  --size".green()
        );
        println!(
            " {:<15} Example for adding password\n",
            "-e  --example".green()
        );
    }

    fn add_example(&self) {
        self.add_help();
        println!("{}", "EXAMPLE:".yellow());
        print!(" pm.exe {} {} ", "add".yellow(), "Google".green());
        print!("{} {}\n", "-s".yellow(), "32".green());
    }
}
