use std::{
    fs::{self, File},
    io::Write,
};

use crossterm::style::Color;

use super::args::Args;
use crate::{
    helpers,
    models::{args, password::Password},
    print,
};

///USAGE:
///pm env set
///
/// or
///
///pm env get
pub struct Env {
    arguments: Args,
}

impl Env {
    ///It runs env
    ///
    ///If it is set then run that function
    ///
    ///If it is get print the Key
    fn env_run(&self) {
        let arg = self.arguments.arguments(2).unwrap().to_lowercase();

        if arg == "set" {
            return self.set_env_secret_key();
        }

        if arg == "get" {
            return helpers::print_with_color_and_bold_line(
                Color::Yellow,
                &std::env::var("SECRET_KEY").unwrap(),
            );
        }

        if arg == "generate" {
            return self.generate_secret_key();
        }
    }

    ///It will generate new secret key
    ///
    /// Writes the secret key to the .env file
    fn generate_secret_key(&self) {
        let mut password = Password::new(&String::from("Hash"), 32);
        password.generate_password().unwrap();
        password.decrypt();
        let password = password.get_password();

        let confirm = helpers::confirm(Color::Red,
        "After setting a secret key, all the password will delete and old secret key will not be access");
        if !confirm {
            return;
        }
        let path = helpers::exe_location();
        fs::write(path + ".env", format!("SECRET_KEY=\"{}\"", password)).unwrap();
    }

    ///It sets new secret key to ".env" file
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

    ///Reads the secret key from ".env" file
    ///
    ///If file not exist then ask user the create a .env file
    pub fn read_from_env(path: &str) {
        let result = dotenv::from_path(path);

        if let Err(error) = result {
            if error.not_found() {
                helpers::print_with_color_and_bold_line(Color::Red, ".env file not found");
                if helpers::confirm(Color::Yellow, "Would you like to create env file") {
                    println!();
                    Env::create_env_file(path);
                    return Env::read_from_env(path);
                }
            }

            helpers::print_with_color_and_bold_line(Color::Red, &error.to_string());
            std::process::exit(1);
        }
    }

    ///Checks the secret key that is stored in ".env" file
    ///
    ///If nothing specified for secret key
    /// use the default one
    pub fn check_secret_key() {
        let secret_key = std::env::var("SECRET_KEY").unwrap();
        if secret_key == "" {
            helpers::print_with_color_and_bold_line(
                Color::Red,
                "No secret key found default will use (not recommended)!",
            );

            helpers::print_with_color_and_bold(Color::Yellow, "To see how to change secret key:");

            helpers::print_with_color_and_bold_line(Color::Magenta, " pm env --help");
            std::env::set_var(
                "SECRET_KEY",
                "W?Xa8Q?E>7g3A=O)s6n6N8>s6L3P6pZ2V>n-CwSv$F(1_1)BlO[0x5p$x_a4d4u&",
            );
            println!();
        }
    }

    ///Creates ".env" file from exe path
    ///
    ///Write "SECRET_KEY="
    fn create_env_file(path: &str) {
        let mut file = File::create(path).expect("Error encountered while creating file!");
        file.write_all(b"SECRET_KEY=\"\"").unwrap();
    }
}

impl args::Arguments for Env {
    ///Returns instance of Args struct
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }

    ///Run for env argument
    fn run(&mut self, _: &sqlite::Connection) {
        if self.arguments.get_len() == 1 {
            self.arguments.get_from_console("Enter env command:");
        }
        if self.check_second_arg() {
            return;
        };
        self.env_run();
    }

    ///Checks second argument which can be "help" or "example"
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

    ///Runs help function for env argument
    fn help(&self) {
        print::env::print_env_help();
    }

    ///Runs help first then example for env argument
    fn example(&self) {
        self.help();
        print::env::print_env_example();
    }
}
