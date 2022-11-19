use colored::Colorize;
use sqlite::State;

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
        .next()
        .unwrap();

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("{} is is already added", name)),
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

pub fn get_one_password(password_name: String, connection: &sqlite::Connection) {
    let query = format!("SELECT * FROM PASSWORDS WHERE NAME = ?");
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, password_name.as_str())).unwrap();

    let mut flag: u8 = 0;
    let mut password = String::new();
    while let Ok(State::Row) = statement.next() {
        flag += 1;
        if flag == 2 {
            break;
        }
        println!(
            "{:<15} {:<15} {:<15}",
            statement.read::<i64, _>("Id").unwrap(),
            statement.read::<String, _>("Name").unwrap(),
            "********"
        );
        password = statement.read::<String, _>("Password").unwrap();
    }
    if flag == 0 {
        return println!("{}", "Password not found".red());
    }
    println!("{}", password);
}
