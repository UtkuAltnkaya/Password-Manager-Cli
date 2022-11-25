pub fn delete_one_password(
    password_name: String,
    connection: &sqlite::Connection,
) -> Result<(), String> {
    let mutation = "DELETE FROM PASSWORDS WHERE UPPER(NAME) = ?";

    let result = connection
        .prepare(mutation)
        .unwrap()
        .into_iter()
        .bind((1, password_name.to_uppercase().as_str()))
        .unwrap()
        .next();

    match result {
        Some(_) => Err(String::from(password_name + " could not delete")), //TODO -> it not shows the error
        None => Ok(()),
    }
}

pub fn delete_all_passwords(connection: &sqlite::Connection) -> Result<(), String> {
    let mutation = "DELETE FROM PASSWORDS";

    let result = connection.prepare(mutation).unwrap().into_iter().next();

    match result {
        Some(_) => Err(String::from("Passwords could not delete")),
        None => Ok(()),
    }
}
