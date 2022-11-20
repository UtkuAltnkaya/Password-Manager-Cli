use password_manager::{args::args::Args, db::db::DbConnection};

fn main() {
    let db = DbConnection::new("db/table.db"); //TODO
    let connection = db.get_connection();
    let args = Args::new();
    args.run(connection);
}
