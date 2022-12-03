use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    style::Color,
    terminal::{Clear, ClearType},
};

use crate::{
    args::{add::Add, args::Args, delete::Delete, list, show::Show, update::Update},
    helpers,
    models::{args::Arguments, menu::Menu},
    print,
};

impl Menu {
    pub fn new() -> Self {
        Self {
            menu_items: vec![
                String::from("Add Password"),
                String::from("Show Password"),
                String::from("List Passwords"),
                String::from("Update Password"),
                String::from("Delete Password"),
                String::from("Help"),
                String::from("Exit"),
            ],
            index: 0,
        }
    }

    pub fn run(&mut self, connection: &sqlite::Connection) {
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::FromCursorDown)).unwrap();
        self.show_menu_items();
        execute!(stdout(), crossterm::cursor::Hide).unwrap();
        loop {
            let event = read().unwrap();
            if let Event::Key(keys) = event {
                execute!(stdout(), MoveTo(0, 0), Clear(ClearType::FromCursorDown)).unwrap();

                match keys.code {
                    KeyCode::Up => self.key_up(),
                    KeyCode::Down => self.key_down(),
                    KeyCode::Esc => return,
                    KeyCode::Enter => return self.run_menu_item(connection),
                    _ => self.show_menu_items(),
                }
            }
        }
    }

    fn run_menu_item(&self, connection: &sqlite::Connection) {
        let mut args = Args::new();
        let first_argument = args.arguments(1).unwrap_or_default();
        execute!(stdout(), crossterm::cursor::Show).unwrap();
        match self.index {
            0 => {
                self.modify_first_argument(&mut args, String::from("add"), &first_argument);
                Add::new(args).run(connection)
            }
            1 => {
                self.modify_first_argument(&mut args, String::from("show"), &first_argument);
                Show::new(args).run(connection)
            }
            2 => {
                self.modify_first_argument(&mut args, String::from("list"), &first_argument);
                list::lists_password(connection)
            }
            3 => {
                self.modify_first_argument(&mut args, String::from("update"), &first_argument);
                Update::new(args).run(connection)
            }
            4 => {
                self.modify_first_argument(&mut args, String::from("delete"), &first_argument);
                Delete::new(args).run(connection)
            }
            5 => {
                self.modify_first_argument(&mut args, String::from("--help"), &first_argument);
                print::help::display_help()
            }
            6 => return,
            _ => (),
        };
    }

    ///It modifies the first argument
    ///
    /// If user enter the menu and select the item and replace the first argument with proper one
    ///
    /// If user use only pm and select the item and add the first argument
    fn modify_first_argument(&self, arguments: &mut Args, arg_name: String, first_args: &str) {
        if first_args == "" {
            return arguments.insert_arguments(1, arg_name).unwrap();
        }
        arguments.replace_argument(1, arg_name).unwrap();
    }

    fn key_up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
        self.show_menu_items()
    }

    fn key_down(&mut self) {
        if self.index < self.menu_items.len() - 1 {
            self.index += 1;
        }
        self.show_menu_items()
    }

    fn show_menu_items(&self) {
        helpers::print_with_color_and_bold_line(Color::Yellow, "--------------Menu--------------");

        for (i, item) in self.menu_items.iter().enumerate() {
            if i == self.index {
                helpers::print_with_color_and_bold_line(
                    Color::Blue,
                    &format!("➜ {}", item).to_owned(),
                );
                continue;
            }
            println!("  {}", item);
        }
    }
}
