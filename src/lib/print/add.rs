use colored::Colorize;

pub fn print_add_help() {
    println!(
        "{}\n pm.exe add {} [--] {}\n",
        "USAGE:".yellow(),
        "[OPTIONS]".green(),
        "[args]",
    );
    println!(
        "{}\n {: <15} Enter site to add password\n",
        "OPTIONS:".yellow(),
        "[SITE-NAME]".green()
    );
    println!(
        "{}\n {:<15} Specify size of password (Multiples of 4 and 4, up to 128)",
        "ARGS:".yellow(),
        "-s  --size".green()
    );
    println!(
        " {:<15} Example for adding password\n",
        "-e  --example".green()
    );
}

pub fn print_add_example() {
    println!("{}", "EXAMPLE:".yellow());
    print!(" pm.exe {} {} ", "add".yellow(), "Google".green());
    print!("{} {}\n", "-s".yellow(), "32".green());
}
