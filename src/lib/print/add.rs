use crossterm::style::Color;

use crate::helpers;

pub fn print_add_help() {
    //Usage
    helpers::print_with_color_line(Color::Yellow, String::from("USAGE:"));
    print!(" pm.exe add ");
    helpers::print_with_color(Color::Green, String::from("[OPTIONS]"));
    println!(" [--] [args]\n");

    //Option
    helpers::print_with_color_line(Color::Yellow, String::from("OPTIONS:"));
    helpers::print_with_color(Color::Green, format!("{: <15}", "[SITE-NAME]"));
    println!(" Enter site to add password\n");

    //Args
    helpers::print_with_color_line(Color::Yellow, String::from("ARGS:"));
    //Args-1
    helpers::print_with_color(Color::Green, format!("{: <15}", "-s  --size"));
    println!(" Specify size of password (Multiples of 4 and 4, up to 128)");

    //Args-2
    helpers::print_with_color(Color::Green, format!("{:<15}", "-e  --example"));
    println!(" Example for adding password\n");
}

pub fn print_add_example() {
    helpers::print_with_color_line(Color::Yellow, String::from("Example:"));
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, String::from("add"));
    helpers::print_with_color(Color::Green, String::from(" Google "));
    helpers::print_with_color(Color::Yellow, String::from("-s "));
    helpers::print_with_color_line(Color::Green, String::from("32"));
}
