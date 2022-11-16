use super::args::Args;
use crate::generate_password::GeneratePassword;
use colored::Colorize;

#[derive(Default)]
pub struct Add {
    arguments: Args,
}

impl Add {
    pub fn new(arguments: Args) {
        Self { arguments }.add_password();
    }
    pub fn add_password(&mut self) {
        if self.arguments.get_len() == 1 {
            self.arguments
                .get_from_console("Enter password name to add:");
        }
        if self.check_second_arg() {
            return;
        };
        self.run(self.arguments.arguments(2), self.check_third_args());
    }

    fn run(&self, second_argument: String, size: usize) {
        match GeneratePassword::new(second_argument, size).generate_password() {
            Ok(result) => println!("{}", result.green()),
            Err(error) => panic!("{}", error.red()),
        };
    }
    fn check_second_arg(&self) -> bool {
        match self.arguments.arguments(2).as_str() {
            "--help" => {
                self.add_help();
                return true;
            }
            "-e" | "--example" => {
                self.add_help();
                println!("{}", "Example:".yellow());
                print!(" pm.exe {} {} ", "add".yellow(), "Google".green());
                print!("{} {}\n", "-s".yellow(), "32".green());
                return true;
            }
            _ => {
                return false;
            }
        }
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
            " {:<15} Example for adding password",
            "-e  --example".green()
        );
    }
}
