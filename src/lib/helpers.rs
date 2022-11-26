use std::io::{self, stdout, Write};

use crossterm::{
    execute,
    style::{self, Color},
};

pub fn input_and_output(color: Color, print_line: &str) -> String {
    let mut line: String = String::new();
    print_with_color(color, print_line.to_owned());

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    line = line.trim_end().to_string();
    line
}

pub fn confirm(color: Color, print_line: String) -> bool {
    print_with_color_line(color, print_line);
    if input_and_output(color, "To confirm press (y):").to_lowercase() == "y" {
        return true;
    }
    return false;
}
pub fn print_with_color(color: Color, content: String) {
    execute!(
        stdout(),
        style::SetForegroundColor(color),
        style::Print(content),
        style::ResetColor
    )
    .unwrap();
}

pub fn print_with_color_line(color: Color, content: String) {
    print_with_color(color, content);
    println!();
}

pub fn print_with_color_and_bold(color: Color, content: String) {
    execute!(
        stdout(),
        style::SetForegroundColor(color),
        style::SetAttribute(style::Attribute::Bold),
        style::Print(content),
        style::ResetColor
    )
    .unwrap();
}

pub fn print_with_color_and_bold_line(color: Color, content: String) {
    print_with_color_and_bold(color, content);
    println!()
}
