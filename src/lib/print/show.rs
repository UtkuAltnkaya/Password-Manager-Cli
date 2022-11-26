use crossterm::style::Color;

use crate::helpers;

pub fn print_show_help() {
    helpers::print_with_color_line(Color::Yellow, "USAGE:");
    print!(" pm.exe show ");
    helpers::print_with_color(Color::Green, "[OPTIONS]");
    println!(" [--] [args]\n");

    helpers::print_with_color_line(Color::Yellow, "OPTIONS:");
    helpers::print_with_color(Color::Green, &format!("{: <15}", "[SITE-NAME]"));
    println!(" Enter site to show password\n");

    helpers::print_with_color_line(Color::Yellow, "ARGS:");
    helpers::print_with_color(Color::Green, &format!("{: <15}", "-c  --copy"));
    println!(" To copy password");

    helpers::print_with_color(Color::Green, &format!("{:<15}", "-h  --help"));
    println!(" Opens helps menu for show password");
    helpers::print_with_color(Color::Green, &format!("{:<15}", "-e  --example"));
    println!(" Example for show password\n");
}

pub fn print_show_example() {
    helpers::print_with_color_line(Color::Yellow, "EXAMPLE:");

    helpers::print_with_color(Color::Blue, "[1]");
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, "show");
    helpers::print_with_color_line(Color::Green, " Google");

    helpers::print_with_color(Color::Blue, "[2]");
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, "show");
    helpers::print_with_color(Color::Green, " Google ");
    helpers::print_with_color_line(Color::Yellow, "-c");
}
