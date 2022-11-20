use crossterm::style::Color;

use crate::{helpers, password::db_password};

pub fn lists_password(connection: &sqlite::Connection) {
    helpers::print_with_color_line(
        Color::Yellow,
        format!("{:<15} {:<15} {:<15}", "Id", "Name", "Password"),
    );

    db_password::list_all_passwords(connection)
}
