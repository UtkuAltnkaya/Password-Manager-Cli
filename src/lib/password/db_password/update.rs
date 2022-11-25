use crate::models::password::Password;

pub fn update_password_name(
    password_name: String,
    new_password_name: String,
    connection: &sqlite::Connection,
) -> Result<(), String> {
    let mutation = "UPDATE PASSWORDS SET NAME = ? WHERE UPPER(NAME) = ?";
    let result = connection
        .prepare(mutation)
        .unwrap()
        .into_iter()
        .bind((1, new_password_name.as_str()))
        .unwrap()
        .bind((2, password_name.to_uppercase().as_str()))
        .unwrap()
        .next();

    match result {
        Some(_) => Err(String::from(new_password_name + " already exist")),
        None => Ok(()),
    }
}

pub fn update_password(
    password_name: String,
    size: usize,
    connection: &sqlite::Connection,
) -> Result<(), String> {
    let mutation = "UPDATE PASSWORDS SET PASSWORD = ? WHERE UPPER(NAME) = ?";
    let mut password_obj = Password::new(password_name.clone(), size);

    if let Err(error) = password_obj.generate_password() {
        return Err(error);
    }

    connection
        .prepare(mutation)
        .unwrap()
        .into_iter()
        .bind((1, password_obj.get_password().as_str()))
        .unwrap()
        .bind((2, password_name.to_uppercase().as_str()))
        .unwrap()
        .next();

    return Ok(());
}
