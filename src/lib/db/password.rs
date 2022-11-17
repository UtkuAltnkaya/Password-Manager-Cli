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

pub fn list_all_passwords(connection: &sqlite::Connection) {
    let query = "SELECT Id,Name FROM PASSWORDS";
    connection
        .iterate(query, |pairs| {
            for &(_, value) in pairs.iter() {
                print!("{:<15} ", value.unwrap())
            }
            println!("{:<15}", "********");
            true
        })
        .unwrap();
}
