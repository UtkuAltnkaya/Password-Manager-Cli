#[derive(Default)]
pub struct Password {
    pub(crate) password_name: String,
    pub(crate) password: String,
    pub(crate) len: usize,
}
