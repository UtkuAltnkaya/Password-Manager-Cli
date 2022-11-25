use crate::models::password::Password;

impl Password {
    pub fn new(password_name: String, size: usize) -> Self {
        Self {
            password_name: password_name,
            password: "".to_owned(),
            len: size,
        }
    }

    pub fn new_with_password(password_name: String, password: String) -> Self {
        let len = password.len();
        Self {
            password_name,
            password,
            len,
        }
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn get_len(&self) -> &usize {
        &self.len
    }
    pub fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    pub fn get_password_name(&self) -> &String {
        &self.password_name
    }

    pub fn set_password_name(&mut self, password_name: String) {
        self.password_name = password_name;
    }
}
