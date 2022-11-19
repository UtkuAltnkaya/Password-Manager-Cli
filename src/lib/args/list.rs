use colored::Colorize;

use crate::password::db_password;

pub fn lists_password(connection: &sqlite::Connection) {
    println!(
        "{:<15} {:<15} {:<15}",
        "Id".yellow(),
        "Name".yellow(),
        "Password".yellow()
    );
    db_password::list_all_passwords(connection)
}
