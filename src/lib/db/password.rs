pub fn add_password_to_db(
    connection: &sqlite::Connection,
    name: String,
    password: String,
) -> Result<(), String> {
    let mutation = "INSERT INTO PASSWORDS (NAME, PASSWORD) VALUES (?, ?);";
    connection
        .prepare(mutation)
        .unwrap()
        .into_iter()
        .bind((1, name.as_str()))
        .unwrap()
        .bind((2, password.as_str()))
        .unwrap()
        .next();
    Ok(())
}
