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
        Some(_) => Err(format!("{} is is already added try different name", name)),
        None => Ok(()),
    }
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

pub fn get_one_password(
    password_name: String,
    connection: &sqlite::Connection,
) -> Result<Password, String> {
    let query = format!("SELECT * FROM PASSWORDS WHERE NAME = ?");
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, password_name.as_str())).unwrap();
    let mut flag: u8 = 0;
    let mut password_obj: Password = Password::new(password_name, 32);
    while let Ok(State::Row) = statement.next() {
        flag += 1;
        if flag == 2 {
            break;
        }
        let password = statement.read::<String, _>("Password").unwrap();
        password_obj.set_password(password.clone());
        password_obj.set_len(password.len());
    }

    if flag == 0 {
        return Err("Password not found".to_owned());
    }
    Ok(password_obj)
}
