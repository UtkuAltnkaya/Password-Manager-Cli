pub struct DbConnection {
    connection: sqlite::Connection,
}

impl DbConnection {
    pub fn new(file_name: &str) -> Self {
        let connection = sqlite::open(&file_name).unwrap();
        Self { connection }
    }

    pub fn get_connection(&self) -> &sqlite::Connection {
        &self.connection
    }
}
