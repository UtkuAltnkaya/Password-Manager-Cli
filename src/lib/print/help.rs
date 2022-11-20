use std::collections::BTreeMap;

use console::style;

pub fn display_help() {
    println!(
        "{} {}\nCli for managing passwords\n",
        style("Password Manager").yellow(),
        style("0.1.0").green()
    );
    println!(
        "{}\n {} <SUBCOMMAND> <SUBCOMMAND>\n",
        style("USAGE:").yellow(),
        "pm.exe",
    );
    println!(
        "{}\n {: <15} Print help information\n {:<15} Print version information\n",
        style("OPTIONS:").yellow(),
        style("-h, --help").green(),
        style("-v, --version").green(),
    );
    println!("{}", style("SUBCOMMAND:").yellow());
    for items in init_b_tree_map().iter() {
        println!(" {:<15} {}", style(items.0).green(), items.1)
    }
}

fn init_b_tree_map() -> BTreeMap<String, String> {
    let mut arguments = BTreeMap::new();
    arguments.insert(
        "Help".to_owned(),
        "Print help message or help of the given subcommand".to_owned(),
    );
    arguments.insert("Add".to_owned(), "Generate new password".to_owned());
    arguments.insert("Delete".to_owned(), "Delete existing password".to_owned());
    arguments.insert("Update".to_owned(), "Update existing password".to_owned());
    arguments.insert("List".to_owned(), "List all password".to_owned());
    arguments.insert("Show".to_owned(), "Show one password".to_owned());
    return arguments;
}
