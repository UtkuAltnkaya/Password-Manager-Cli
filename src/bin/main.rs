use password_manager::{
    args::{args::Args, env::Env},
    config::db::DbConnection,
    helpers,
};

fn main() {
    let path = helpers::exe_location();
    Env::read_from_env(&format!("{}.env", &path));
    Env::check_secret_key();

    let db = DbConnection::new(&(path + r"db\table.db"));
    db.create_password_table();
    let connection = db.get_connection();

    let args = Args::new();
    args.run(connection);
}
