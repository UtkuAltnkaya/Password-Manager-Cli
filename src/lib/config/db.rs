use std::fs::{self, File};

use crossterm::style::Color;

use crate::helpers;

pub struct DbConnection {
    connection: sqlite::Connection,
}

impl DbConnection {
    pub fn new(path: &str) -> Self {
        let connection = DbConnection::check_db_location(path).unwrap();
        Self { connection }
    }

    pub fn get_connection(&self) -> &sqlite::Connection {
        &self.connection
    }

    fn check_db_location(path: &str) -> Result<sqlite::Connection, String> {
        let result = sqlite::open(path);
        match result {
            Ok(connection) => Ok(connection),
            Err(error) => {
                if error.code.unwrap() == 14 {
                    //unable to open database file
                    helpers::print_with_color_and_bold_line(
                        Color::Red,
                        "Database path and file not found",
                    );
                    if helpers::confirm(Color::Blue, "Would you like to create one:") {
                        DbConnection::create_dir_an_db();
                        return DbConnection::check_db_location(path);
                    }
                }
                Err(error.to_string())
            }
        }
    }

    fn create_dir_an_db() {
        let mut path = helpers::exe_location();
        path += "/db";
        fs::create_dir(&path).unwrap();
        path += "/table.db";
        File::create(&path).expect("Error encountered while creating file!");
    }
}
