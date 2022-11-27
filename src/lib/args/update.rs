use crossterm::style::Color;

use crate::{helpers, models::args, password::db_password, print};

use super::args::Args;

///USAGE:
///pm update "PASSWORD-NAME" name "NEW-PASSWORD-NAME"
///
/// or
///pm update "PASSWORD-NAME" password
///
///or
///
///pm update "PASSWORD-NAME" size "NEW-PASSWORD-SIZE"
pub struct Update {
    arguments: Args,
}
impl Update {
    ///It updates for proper argument which can be "name","password","size".
    pub fn update_password(&mut self, connection: &sqlite::Connection) {
        let third = self.arguments.arguments(3).unwrap();
        match third.to_lowercase().as_str() {
            "name" => {
                self.check_fourth_arg(&third);
                self.result(db_password::update::update_password_name(
                    self.arguments.arguments(2).unwrap(),
                    self.arguments.arguments(4).unwrap(),
                    connection,
                ))
            }
            "password" => {
                if !helpers::confirm(Color::Red, "After update, old password will be deleted") {
                    return;
                }
                self.result(db_password::update::update_password(
                    self.arguments.arguments(2).unwrap(),
                    32,
                    connection,
                ))
            }
            "size" => {
                self.check_fourth_arg(&third);
                let size: Result<usize, _> = self.arguments.arguments(4).unwrap().parse();
                if let Err(_) = size {
                    return helpers::print_with_color_and_bold_line(
                        Color::Red,
                        "Enter valid argument",
                    );
                }
                if !helpers::confirm(Color::Red, "After update, old password will be deleted") {
                    return;
                }
                self.result(db_password::update::update_password(
                    self.arguments.arguments(2).unwrap(),
                    size.unwrap(),
                    connection,
                ))
            }
            _ => {}
        }
    }

    ///If password updated successfully print the result.
    ///
    ///If not print the error.
    fn result(&self, result: Result<(), String>) {
        match result {
            Ok(()) => helpers::print_with_color_and_bold_line(Color::Green, "Password updated"),
            Err(error) => helpers::print_with_color_and_bold_line(Color::Red, &error),
        }
    }

    ///Checks the third argument which can be "name","password","size"
    ///
    ///If not entered get from cli.
    fn check_third_arg(&mut self) {
        if let Err(_) = self.arguments.arguments(3) {
            let input =
                helpers::input_and_output(Color::Grey, "Select option (name,password,size):");
            self.arguments.insert_arguments(3, input).unwrap();
            self.check_third_arg();
        }
    }

    ///Checks the fourth argument which can be "NEW-PASSWORD-NAME" or "NEW-PASSWORD-SIZE"
    ///
    ///If not entered get from cli
    fn check_fourth_arg(&mut self, operation_type: &String) {
        if let Err(_) = self.arguments.arguments(4) {
            let input =
                helpers::input_and_output(Color::Grey, &format!("Enter new {}:", operation_type));
            self.arguments.insert_arguments(4, input).unwrap();
            self.check_fourth_arg(operation_type);
        }
    }
}

///Trait implementation for Update
impl args::Arguments for Update {
    ///Returns instance of Update struct
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }

    ///It checks user entered the password name to update
    ///If not get from cli
    fn run(&mut self, connection: &sqlite::Connection) {
        if self.arguments.get_len() == 1 {
            self.arguments
                .get_from_console("Enter password name to update:");
        }
        if self.check_second_arg() {
            return;
        }
        self.check_third_arg();
        self.update_password(connection);
    }

    ///Checks second argument which can be "help" or "example"
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

    ///It runs help function for show argument
    fn help(&self) {
        print::update::print_add_help()
    }

    ///It runs help function first then runs example for show argument
    fn example(&self) {
        self.help();
        print::update::print_add_example();
    }
}
