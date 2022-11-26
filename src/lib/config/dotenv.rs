use std::{fs::File, io::Write};

use crossterm::style::Color;

use crate::helpers;

pub fn read_from_env() {
    let result = dotenv::dotenv();

    if let Err(error) = result {
        if error.not_found() {
            helpers::print_with_color_and_bold_line(Color::Red, ".env file not found");
            if helpers::confirm(Color::Yellow, "Would you like to create env file") {
                create_env_file();
                dotenv::dotenv().unwrap();
                return;
            }
        }

        helpers::print_with_color_and_bold_line(Color::Red, error.to_string().as_str());
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

fn create_env_file() {
    let mut file = File::create(".env").expect("Error encountered while creating file!");
    file.write_all(b"SECRET_KEY=\"\"").unwrap();
}
