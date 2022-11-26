use crossterm::style::Color;

use crate::helpers;

pub fn print_env_help() {
    helpers::print_with_color_line(Color::Yellow, String::from("USAGE:"));
    print!(" pm.exe env ");
    helpers::print_with_color(Color::Green, String::from("[OPTIONS]"));
    println!(" [--] [args]\n");

    helpers::print_with_color_line(Color::Yellow, String::from("OPTIONS:"));
    helpers::print_with_color(Color::Green, format!("{: <15}", "Set"));
    println!(" Set new secret key to env file");
    helpers::print_with_color(Color::Green, format!("{: <15}", "Show"));
    println!(" Show secret key\n");

    helpers::print_with_color_line(Color::Yellow, String::from("ARGS:"));

    helpers::print_with_color(Color::Green, format!("{:<15}", "-h  --help"));
    println!(" Opens helps menu for show password");
    helpers::print_with_color(Color::Green, format!("{:<15}", "-e  --example"));
    println!(" Example for show password\n");
}

pub fn print_env_example() {
    helpers::print_with_color_line(Color::Yellow, String::from("EXAMPLE:"));

    helpers::print_with_color(Color::Blue, String::from("[1]"));
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, String::from("env"));
    helpers::print_with_color_line(Color::Green, String::from(" set"));

    helpers::print_with_color(Color::Blue, String::from("[2]"));
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, String::from("env"));
    helpers::print_with_color_line(Color::Green, String::from(" get "));
}
