use crossterm::style::Color;

use crate::{helpers, models::password::Password, password::db_password};

pub fn lists_password(connection: &sqlite::Connection) {
    let result = db_password::get_all_passwords(connection);
    match result {
        Ok(password_list) => print_passwords(password_list),
        Err(error) => helpers::print_with_color_line(Color::Red, &error),
    }
}

fn print_passwords(password_list: Vec<Password>) {
    helpers::print_with_color_and_bold_line(
        Color::Yellow,
        &format!("{:<15} {:<15} {:<15}", "Id", "Name", "Password"),
    );

    for (i, item) in password_list.iter().enumerate() {
        println!(
            "{:<15} {:<15} {:<15}",
            i + 1,
            item.get_password_name(),
            "********"
        );
    }
}
