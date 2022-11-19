use colored::Colorize;

use crate::db::password;

pub fn lists_password(connection: &sqlite::Connection) {
    println!(
        "{:<15} {:<15} {:<15}",
        "Id".yellow(),
        "Name".yellow(),
        "Password".yellow()
    );
    //password::list_all_passwords(connection)
    password::get_one_password("g".to_owned(), connection)
}
