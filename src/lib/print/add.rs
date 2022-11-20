use console::style;

pub fn print_add_help() {
    println!(
        "{}\n pm.exe add {} [--] {}\n",
        style("USAGE").yellow(),
        style("[OPTIONS]").green(),
        "[args]",
    );
    println!(
        "{}\n {: <15} Enter site to add password\n",
        style("OPTIONS:").yellow(),
        style("[SITE-NAME]").green(),
    );
    println!(
        "{}\n {:<15} Specify size of password (Multiples of 4 and 4, up to 128)",
        style("ARGS:").yellow(),
        style("-s  --size").green(),
    );
    println!(
        " {:<15} Example for adding password\n",
        style("-e  --example").green(),
    );
}

pub fn print_add_example() {
    println!("{}", style("EXAMPLE:").yellow());
    print!(
        " pm.exe {} {} ",
        style("add").yellow(),
        style("Google").green()
    );
    print!("{} {}\n", style("-s").yellow(), style("32").green());
}
