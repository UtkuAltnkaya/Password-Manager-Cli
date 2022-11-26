use crossterm::style::Color;

use crate::helpers;

pub fn print_add_help() {
    //Usage
    helpers::print_with_color_line(Color::Yellow, "USAGE:");
    print!(" pm.exe update ");
    helpers::print_with_color(Color::Green, "[OPTIONS]");
    helpers::print_with_color(Color::Yellow, " [UPDATE-OPTIONS]");
    println!(" [--] [args]\n");

    //Option
    helpers::print_with_color_line(Color::Yellow, "OPTIONS:");
    helpers::print_with_color(Color::Green, &format!("{:<20}", "[SITE-NAME]"));
    println!(" Enter site to update\n");

    //Update Options
    helpers::print_with_color_line(Color::Yellow, "UPDATE-OPTIONS:");
    //U-option-1
    helpers::print_with_color(Color::Green, &format!("{: <20}", "Name"));
    println!(" It will change the password name");
    //U-option-2
    helpers::print_with_color(Color::Green, &format!("{: <20}", "Password"));
    println!(" It will generate new password with default size (32)");
    //U-option-3
    helpers::print_with_color(Color::Green, &format!("{: <20}", "Size"));
    println!(" It will generate new password with new size\n");

    //Args
    helpers::print_with_color_line(Color::Yellow, "ARGS:");
    //Args-1
    helpers::print_with_color(Color::Green, &format!("{:<20}", "-h  --help"));
    println!(" Opens help menu for update password");
    //Args-2
    helpers::print_with_color(Color::Green, &format!("{:<20}", "-e  --example"));
    println!(" Example for adding password\n");
}

pub fn print_add_example() {
    helpers::print_with_color_line(Color::Yellow, "EXAMPLE:");

    helpers::print_with_color(Color::Blue, "[1]");
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, "update");
    helpers::print_with_color(Color::Green, " Google ");
    helpers::print_with_color(Color::Yellow, "name ");
    helpers::print_with_color_line(Color::Green, "Google-Updated");

    helpers::print_with_color(Color::Blue, "[2]");
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, "update");
    helpers::print_with_color(Color::Green, " Google ");
    helpers::print_with_color_line(Color::Yellow, "password ");

    helpers::print_with_color(Color::Blue, "[3]");
    print!(" pm.exe ");
    helpers::print_with_color(Color::Yellow, "update");
    helpers::print_with_color(Color::Green, " Google ");
    helpers::print_with_color(Color::Yellow, "size ");
    helpers::print_with_color_line(Color::Green, "64");
}
