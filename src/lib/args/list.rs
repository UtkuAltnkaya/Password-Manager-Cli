use console::style;

use crate::password::db_password;

pub fn lists_password(connection: &sqlite::Connection) {
    println!(
        "{:<15} {:<15} {:<15}",
        style("Id").yellow(),
        style("Name").yellow(),
        style("Password").yellow()
    );
    db_password::list_all_passwords(connection)
}
