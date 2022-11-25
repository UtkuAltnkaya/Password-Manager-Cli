use crossterm::style::Color;

use crate::{
    helpers,
    models::{args, password::Password},
    password::db_password,
};

use super::args::Args;

pub struct Update {
    arguments: Args,
}
//pm update github name Github
impl Update {
    pub fn update_password(&mut self, connection: &sqlite::Connection) {
        let third_arg = self.arguments.arguments(3);
        match third_arg {
            Ok(result) => {
                if result == "name" {
                    self.check_fourth_arg(&result);
                    return self.result(db_password::update::update_password_name(
                        self.arguments.arguments(2).unwrap(),
                        self.arguments.arguments(4).unwrap(),
                        connection,
                    ));
                }
                if result == "password" {
                    if !self.confirm() {
                        return;
                    }
                    return self.result(db_password::update::update_password(
                        self.arguments.arguments(2).unwrap(),
                        32,
                        connection,
                    ));
                }

                if result == "size" {
                    self.check_fourth_arg(&result);
                    let size: Result<usize, _> = self.arguments.arguments(4).unwrap().parse();
                    if let Err(_) = size {
                        return helpers::print_with_color_and_bold_line(
                            Color::Red,
                            String::from("Enter valid argument"),
                        );
                    }
                    if !self.confirm() {
                        return;
                    }

                    return self.result(db_password::update::update_password(
                        self.arguments.arguments(2).unwrap(),
                        size.unwrap(),
                        connection,
                    ));
                }
            }
            Err(_) => todo!(), //TODO
        }
    }

    fn confirm(&self) -> bool {
        helpers::print_with_color_line(
            Color::Red,
            String::from("After updated old password will be deleted"),
        );
        if helpers::input_and_output(Color::Red, "To confirm enter (y):").to_lowercase() == "y" {
            println!();
            return true;
        }
        return false;
    }

    fn result(&self, result: Result<Password, String>) {
        match result {
            Ok(password) => self.show(password),
            Err(error) => helpers::print_with_color_and_bold_line(Color::Red, error),
        }
    }

    fn show(&self, password: Password) {
        helpers::print_with_color_and_bold_line(
            Color::Yellow,
            format!("{:<15} {:<15} {:<15}", "Id", "Name", "Password").to_owned(),
        );
        print!(
            "{:<15} {:<15} {:<15}\n",
            1,
            password.get_password_name(),
            "********"
        );
        helpers::print_with_color_and_bold_line(Color::Green, String::from("Password updated"));
    }

    fn check_third_arg(&mut self) {
        if let Err(_) = self.arguments.arguments(3) {
            let input =
                helpers::input_and_output(Color::Grey, "Select option (name,password,size):");
            self.arguments.insert_arguments(3, input).unwrap();
            self.check_third_arg();
        }
    }

    fn check_fourth_arg(&mut self, operation_type: &String) {
        if let Err(_) = self.arguments.arguments(4) {
            let input =
                helpers::input_and_output(Color::Grey, &format!("Enter new {}:", operation_type));
            self.arguments.insert_arguments(4, input).unwrap();
            self.check_fourth_arg(operation_type);
        }
    }
}

impl args::Arguments for Update {
    fn new(arguments: Args) -> Self {
        Self { arguments }
    }

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

    fn help(&self) {}

    fn example(&self) {}
}
