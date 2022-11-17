use colored::Colorize;

use crate::generate_password;

pub fn lists_password() {
    //connect to db
    println!("{} {:<15}", "Name".yellow(), "Password".yellow());
    // let obj = add::Add::new(self.arguments.clone());

    let mut obj = generate_password::GeneratePassword::new("Amazon".to_owned(), 32);
    obj.generate_password().unwrap();

    println!("{} {:<15}", "Amazon", obj.get_password());
}
