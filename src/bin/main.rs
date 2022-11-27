use std::env;

use password_manager::{
    args::args::Args,
    config::{db::DbConnection, dotenv},
};

fn main() {
    let mut path = env::current_exe().unwrap().display().to_string();
    dotenv::read_from_env();
    dotenv::check_secret_key();

    path = (path.trim_end_matches("pm.exe")).to_string();
    //&(path.to_string() + r"\db\table.db")
    let db = DbConnection::new(&(path + r"\db\table.db")); //TODO
    let connection = db.get_connection();
    let args = Args::new();
    args.run(connection);
}
