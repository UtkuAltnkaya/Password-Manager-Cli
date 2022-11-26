use crossterm::style::Color;

use crate::helpers;

pub fn print_add_help() {
    //Usage
    helpers::print_with_color_line(Color::Yellow, "USAGE:");
    print!(" pm.exe delete ");
    helpers::print_with_color(Color::Green, "[OPTIONS]");
    println!(" [--] [args]\n");

    //Option
    helpers::print_with_color_line(Color::Yellow, "OPTIONS:");
    helpers::print_with_color(Color::Green, &format!("{:<20}", "[SITE-NAME]"));
    println!(" Enter password name to delete\n");

    //Args
    helpers::print_with_color_line(Color::Yellow, "ARGS:");
    //Args-1
    helpers::print_with_color(Color::Green, &format!("{:<20}", "-a --all"));
    println!(" It will delete all password");
    //Args-2
    helpers::print_with_color(Color::Green, &format!("{:<20}", "-h --help"));
    println!(" Opens help menu for delete password");
    //Args-3
    helpers::print_with_color(Color::Green, &format!("{:<20}", "-e  --example"));
    println!(" Example for deleting password\n");
}

pub fn print_add_example() {
    helpers::print_with_color_line(Color::Yellow, "EXAMPLE:");

    helpers::print_with_color(Color::Blue, "[1]");
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, "delete");
    helpers::print_with_color_line(Color::Green, " Google ");

    helpers::print_with_color(Color::Blue, "[2]");
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, "delete");
    helpers::print_with_color_line(Color::Green, " --all ");
}
