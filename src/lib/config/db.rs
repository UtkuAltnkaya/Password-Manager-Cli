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
                    if helpers::confirm(Color::Yellow, "Would you like to create one:") {
                        DbConnection::create_dir_an_db();
                        println!();
                        return DbConnection::check_db_location(path);
                    }
                }
                Err(error.to_string())
            }
        }
    }

    pub fn create_password_table(&self) {
        let statement = "CREATE TABLE IF NOT EXISTS \nPasswords (\n\tId\tINTEGER NOT NULL UNIQUE,\n\tName\tTEXT NOT NULL UNIQUE,\n\tPassword\tINTEGER NOT NULL,\n\tPRIMARY KEY(Id AUTOINCREMENT)\n)";
        self.connection.prepare(statement).unwrap().next().unwrap();
    }

    fn create_dir_an_db() {
        let mut path = helpers::exe_location();
        path += "/db";
        fs::create_dir(&path).unwrap();
        path += "/table.db";
        File::create(&path).expect("Error encountered while creating file!");
    }
}
