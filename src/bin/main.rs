use password_manager::{args::args::Args, db::db::DbConnection};

fn main() {
    let db = DbConnection::new("src/table.db");
    let connection = db.get_connection();
    let mut args = Args::new();
    args.run(connection);
}
