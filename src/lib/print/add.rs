use crossterm::style::Color;

use crate::helpers;

pub fn print_add_help() {
    //Usage
    helpers::print_with_color_line(Color::Yellow, "USAGE:");
    print!(" pm.exe add ");
    helpers::print_with_color(Color::Green, "[OPTIONS]");
    println!(" [--] [args]\n");

    //Option
    helpers::print_with_color_line(Color::Yellow, "OPTIONS:");
    helpers::print_with_color(Color::Green, &format!("{: <15}", "[SITE-NAME]"));
    println!(" Enter site to add password\n");

    //Args
    helpers::print_with_color_line(Color::Yellow, "ARGS:");
    //Args-1
    helpers::print_with_color(Color::Green, &format!("{: <15}", "-s  --size"));
    println!(" Specify size of password (Multiples of 4 and 4, up to 128)");

    //Args-2
    helpers::print_with_color(Color::Green, &format!("{:<15}", "-h  --help"));
    println!(" Opens help menu for add password");

    //Args-3
    helpers::print_with_color(Color::Green, &format!("{:<15}", "-e  --example"));
    println!(" Example for adding password\n");
}

pub fn print_add_example() {
    helpers::print_with_color_line(Color::Yellow, "EXAMPLE:");

    helpers::print_with_color(Color::Blue, "[1]");
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, "add");
    helpers::print_with_color_line(Color::Green, " Google ");

    helpers::print_with_color(Color::Blue, "[2]");
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, "add");
    helpers::print_with_color(Color::Green, " Google ");
    helpers::print_with_color(Color::Yellow, "-s ");
    helpers::print_with_color_line(Color::Green, "64");
}
