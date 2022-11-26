pub mod delete;
pub mod update;

use sqlite::State;

use crate::models::password::Password;

pub fn add_password_to_db(
    connection: &sqlite::Connection,
    name: String,
    password: String,
) -> Result<(), String> {
    let mutation = "INSERT INTO PASSWORDS (NAME, PASSWORD) VALUES (?, ?);";
    let result = connection
        .prepare(mutation)
        .unwrap()
        .into_iter()
        .bind((1, name.as_str()))
        .unwrap()
        .bind((2, password.as_str()))
        .unwrap()
        .next();

    match result {
        Some(_) => Err(format!("{} is is already added try different one", name)),
        None => Ok(()),
    }
}

pub fn get_all_passwords(connection: &sqlite::Connection) -> Result<Vec<Password>, String> {
    let query = "SELECT * FROM PASSWORDS ORDER BY NAME";
    let mut statement = connection.prepare(query).unwrap();
    let mut password_list: Vec<Password> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        let name = statement.read::<String, _>("Name").unwrap();
        let password = statement.read::<String, _>("Password").unwrap();
        password_list.push(Password::new_with_password(name, password));
    }

    if password_list.len() == 0 {
        return Err("Password not found".to_owned());
    }

    return Ok(password_list);
}

pub fn get_one_password(
    password_name: String,
    connection: &sqlite::Connection,
) -> Result<Password, String> {
    let query = "SELECT * FROM PASSWORDS WHERE UPPER(NAME) = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement
        .bind((1, password_name.to_uppercase().as_str()))
        .unwrap();
    let mut flag: u8 = 0;
    let mut password_obj: Password = Password::new(&password_name, 32);

    while let Ok(State::Row) = statement.next() {
        flag += 1;
        if flag == 2 {
            break;
        }
        let name = statement.read::<String, _>("Name").unwrap();
        let password = statement.read::<String, _>("Password").unwrap();
        password_obj.set_password_name(name);
        password_obj.set_password(&password);
        password_obj.set_len(password.len());
    }

    if flag == 0 {
        return Err("Password not found".to_owned());
    }
    Ok(password_obj)
}
