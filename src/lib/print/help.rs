use std::collections::BTreeMap;

use crossterm::style::Color;

use crate::helpers;

pub fn display_help() {
    //Title
    helpers::print_with_color_and_bold(Color::Yellow, String::from("Password Manager "));
    helpers::print_with_color_and_bold_line(Color::Green, String::from("0.1.0"));
    println!("Cli for managing passwords\n");

    //Usage
    helpers::print_with_color_line(Color::Yellow, String::from("USAGE:"));
    println!(" pm.exe <SUBCOMMAND> <SUBCOMMAND>\n");

    //Options
    helpers::print_with_color_line(Color::Yellow, String::from("OPTIONS:"));
    //Options-
    helpers::print_with_color(Color::Green, format!("{:<22}", " -h, --help"));
    println!("Print help information");
    //Options-2
    helpers::print_with_color(Color::Green, format!("{:<22}", " -v, --version"));
    println!("Print version information\n");

    //Subcommands
    helpers::print_with_color_line(Color::Yellow, String::from("SUBCOMMAND:"));
    for items in init_b_tree_map().iter() {
        helpers::print_with_color(Color::Green, format!(" {:<20} ", items.0));
        println!("{}", items.1);
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
    arguments.insert("Menu".to_owned(), "Show Menu".to_owned());
    return arguments;
}
