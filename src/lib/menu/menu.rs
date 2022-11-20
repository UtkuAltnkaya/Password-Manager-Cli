use console::{style, Key, Term};

use crate::{
    args::{add::Add, args::Args, list, show::Show},
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
        let terminal = Term::stdout();
        let mut key = Key::ArrowUp;
        loop {
            terminal.clear_screen().unwrap();
            match key {
                Key::ArrowUp => self.key_up(),
                Key::ArrowDown => self.key_down(),
                Key::Escape => return,
                Key::Enter => return self.run_menu_item(connection),
                _ => self.show_menu_items(),
            }
            key = terminal.read_key().unwrap();
        }
    }

    fn run_menu_item(&self, connection: &sqlite::Connection) {
        let mut args = Args::new();

        match self.index {
            0 => {
                args.insert_arguments(1, String::from("add")).unwrap();
                Add::new(args).run(connection)
            }
            1 => {
                args.insert_arguments(1, String::from("show")).unwrap();
                Show::new(args).run(connection)
            }
            2 => {
                args.insert_arguments(1, String::from("list")).unwrap();
                list::lists_password(connection)
            }
            3 => {}
            4 => {}
            5 => {
                args.insert_arguments(1, String::from("--help")).unwrap();
                print::help::display_help();
            }
            6 => return,
            _ => {}
        };
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
        println!(
            "{}",
            style("--------------Menu--------------").bold().yellow()
        );
        for (i, item) in self.menu_items.iter().enumerate() {
            if i == self.index {
                println!("➜ {}", style(item).bold().blue());
                continue;
            }
            println!("  {}", item);
        }
    }
}
