use crossterm::style::Color;

use crate::helpers;

pub fn print_show_help() {
    helpers::print_with_color_line(Color::Yellow, String::from("USAGE:"));
    print!(" pm.exe show ");
    helpers::print_with_color(Color::Green, String::from("[OPTIONS]"));
    println!(" [--] [args]\n");

    helpers::print_with_color_line(Color::Yellow, String::from("OPTIONS:"));
    helpers::print_with_color(Color::Green, format!("{: <15}", "[SITE-NAME]"));
    println!(" Enter site to show password\n");

    helpers::print_with_color_line(Color::Yellow, String::from("ARGS:"));
    helpers::print_with_color(Color::Green, format!("{: <15}", "-c  --copy"));
    println!(" To copy password");

    helpers::print_with_color(Color::Green, format!("{:<15}", "-e  --example"));
    println!(" Example for show password\n");
}

pub fn print_show_example() {
    helpers::print_with_color_line(Color::Yellow, String::from("Example:"));

    helpers::print_with_color(Color::Blue, String::from("[1]"));
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, String::from("show"));
    helpers::print_with_color(Color::Green, String::from(" Google "));
    helpers::print_with_color_line(Color::Yellow, String::from("-c"));

    helpers::print_with_color(Color::Blue, String::from("[2]"));
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, String::from("show"));
    helpers::print_with_color_line(Color::Green, String::from(" Google"));
}
