use password_manager::{args::args::Args, db::db::DbConnection};

fn main() {
    let db = DbConnection::new("db/table.db");
    let connection = db.get_connection();
    let mut args = Args::new();
    args.run(connection);
}
