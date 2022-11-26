use password_manager::{
    args::args::Args,
    config::{db::DbConnection, dotenv},
};

fn main() {
    dotenv::read_from_env();
    dotenv::check_secret_key();

    let db = DbConnection::new("db/table.db"); //TODO
    let connection = db.get_connection();
    let args = Args::new();
    args.run(connection);
}
