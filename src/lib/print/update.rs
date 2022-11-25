use crossterm::style::Color;

use crate::helpers;

pub fn print_add_help() {
    //Usage
    helpers::print_with_color_line(Color::Yellow, String::from("USAGE:"));
    print!(" pm.exe update ");
    helpers::print_with_color(Color::Green, String::from("[OPTIONS]"));
    helpers::print_with_color(Color::Yellow, String::from(" [UPDATE-OPTIONS]"));
    println!(" [--] [args]\n");

    //Option
    helpers::print_with_color_line(Color::Yellow, String::from("OPTIONS:"));
    helpers::print_with_color(Color::Green, format!("{:<20}", "[SITE-NAME]"));
    println!(" Enter site to update\n");

    //Update Options
    helpers::print_with_color_line(Color::Yellow, String::from("UPDATE-OPTIONS:"));
    //U-option-1
    helpers::print_with_color(Color::Green, format!("{: <20}", "Name"));
    println!(" It will change the password name");
    //U-option-2
    helpers::print_with_color(Color::Green, format!("{: <20}", "Password"));
    println!(" It will generate new password with default size (32)");
    //U-option-3
    helpers::print_with_color(Color::Green, format!("{: <20}", "Size"));
    println!(" It will generate new password with new size\n");

    //Args
    helpers::print_with_color_line(Color::Yellow, String::from("ARGS:"));
    //Args-1
    helpers::print_with_color(Color::Green, format!("{:<20}", "-h  --help"));
    println!(" Opens help menu for update password");
    //Args-2
    helpers::print_with_color(Color::Green, format!("{:<20}", "-e  --example"));
    println!(" Example for adding password\n");
}

pub fn print_add_example() {
    helpers::print_with_color_line(Color::Yellow, String::from("EXAMPLE:"));

    helpers::print_with_color(Color::Blue, String::from("[1]"));
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, String::from("update"));
    helpers::print_with_color(Color::Green, String::from(" Google "));
    helpers::print_with_color(Color::Yellow, String::from("name "));
    helpers::print_with_color_line(Color::Green, String::from("Google-Updated"));

    helpers::print_with_color(Color::Blue, String::from("[2]"));
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, String::from("update"));
    helpers::print_with_color(Color::Green, String::from(" Google "));
    helpers::print_with_color_line(Color::Yellow, String::from("password "));

    helpers::print_with_color(Color::Blue, String::from("[3]"));
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, String::from("update"));
    helpers::print_with_color(Color::Green, String::from(" Google "));
    helpers::print_with_color(Color::Yellow, String::from("size "));
    helpers::print_with_color_line(Color::Green, String::from("64"));
}
