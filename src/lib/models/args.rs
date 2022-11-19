use crate::args::args::Args;

pub trait Arguments {
    fn new(arguments: Args) -> Self;
    fn run(&mut self, connection: &sqlite::Connection);
    fn check_second_arg(&self) -> bool;
    fn help(&self);
    fn example(&self);
}
