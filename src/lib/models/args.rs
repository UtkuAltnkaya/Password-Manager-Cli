pub trait Arguments {
    fn run(&mut self, connection: &sqlite::Connection);
    fn check_second_arg(&self) -> bool;
    fn help(&self);
    fn example(&self);
}
