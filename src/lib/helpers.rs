use std::io::{self, Write};

pub fn input_and_output(print_line: &str) -> String {
    let mut line: String = String::new();
    print!("{}", print_line);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    line = line.trim_end().to_string();
    line
}
